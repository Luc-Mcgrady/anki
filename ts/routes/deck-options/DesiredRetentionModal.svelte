<!--
Copyright: Ankitects Pty Ltd and contributors
License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
-->
<script lang="ts">
    import SettingTitle from "$lib/components/SettingTitle.svelte";
    import Graph from "../graphs/Graph.svelte";
    import HoverColumns from "../graphs/HoverColumns.svelte";
    import CumulativeOverlay from "../graphs/CumulativeOverlay.svelte";
    import AxisTicks from "../graphs/AxisTicks.svelte";
    import NoDataOverlay from "../graphs/NoDataOverlay.svelte";
    import TableData from "../graphs/TableData.svelte";
    import InputBox from "../graphs/InputBox.svelte";
    import { defaultGraphBounds, type TableDatum } from "../graphs/graph-helpers";
    import { SimulateSubgraph, type Point } from "../graphs/simulator";
    import * as tr from "@generated/ftl";
    import { renderSimulationChart } from "../graphs/simulator";
    import { simulateFsrsReview } from "@generated/backend";
    import { runWithBackendProgress } from "@tslib/progress";
    import type {
        SimulateFsrsReviewRequest,
        SimulateFsrsReviewResponse,
    } from "@generated/anki/scheduler_pb";
    import type { DeckOptionsState } from "./lib";
    import { DeckConfig_Config_LeechAction } from "@generated/anki/deck_config_pb";
    import SimulatorConfig from "./SimulatorConfig.svelte";
    import ModalHeader from "./ModalHeader.svelte";
    import SwitchRow from "$lib/components/SwitchRow.svelte";
    import * as _ from "lodash-es";

    export let shown = false;
    export let state: DeckOptionsState;
    export let simulateFsrsRequest: SimulateFsrsReviewRequest;
    export let computing: boolean;
    export let openHelpModal: (key: string) => void;
    export let onPresetChange: () => void;

    const config = state.currentConfig;
    let simulateSubgraph: SimulateSubgraph = SimulateSubgraph.count;
    let tableData: TableDatum[] = [];
    let simulating: boolean = false;
    const bounds = defaultGraphBounds();

    let svg: HTMLElement | SVGElement | null = null;
    let simulationNumber = 0;
    let points: Point[] = [];
    let newCardsIgnoreReviewLimit = state.newCardsIgnoreReviewLimit;
    let smooth = true;
    let suspendLeeches = $config.leechAction == DeckConfig_Config_LeechAction.SUSPEND;
    let leechThreshold = $config.leechThreshold;

    $: daysToSimulate = 365;
    $: deckSize = 0;
    $: windowSize = Math.ceil(daysToSimulate / 365);

    function movingAverage(y: number[], windowSize: number): number[] {
        const result: number[] = [];
        for (let i = 0; i < y.length; i++) {
            let sum = 0;
            let count = 0;
            for (let j = Math.max(0, i - windowSize + 1); j <= i; j++) {
                sum += y[j];
                count++;
            }
            result.push(sum / count);
        }
        return result;
    }

    function addArrays(arr1: number[], arr2: number[]): number[] {
        return arr1.map((value, index) => value + arr2[index]);
    }

    let progress = 0

    async function simulateFsrs(): Promise<void> {
        let resp: SimulateFsrsReviewResponse | undefined;
        simulateFsrsRequest.daysToSimulate = daysToSimulate;
        simulateFsrsRequest.deckSize = deckSize;
        simulateFsrsRequest.suspendAfterLapseCount = suspendLeeches
            ? leechThreshold
            : undefined;
        const begin = 0.7
        const end = 0.99
        const steps = 20
        let results: SimulateFsrsReviewResponse[] = []
        try {
            const retentions = _.range(begin, end, (end - begin) / steps)
            for (const [i, desiredRetention] of retentions.entries()) {
                await runWithBackendProgress(
                    async () => {
                        simulating = true;
                        simulateFsrsRequest.desiredRetention = desiredRetention
                        progress = i
                        results[desiredRetention] = await simulateFsrsReview(simulateFsrsRequest);
                    },
                    () => {},
                );
            }
        } finally {
            simulating = false;
            if (resp) {
                simulationNumber += 1;
                const dailyTotalCount = addArrays(
                    resp.dailyReviewCount,
                    resp.dailyNewCount,
                );

                const dailyMemorizedCount = resp.accumulatedKnowledgeAcquisition;

                points = points.concat(
                    resp.dailyTimeCost.map((v, i) => ({
                        x: i,
                        timeCost: v,
                        count: dailyTotalCount[i],
                        memorized: dailyMemorizedCount[i],
                        label: simulationNumber,
                    })),
                );

                tableData = renderSimulationChart(
                    svg as SVGElement,
                    bounds,
                    points,
                    simulateSubgraph,
                );
            }
        }
    }

    function clearSimulation() {
        points = points.filter((p) => p.label !== simulationNumber);
        simulationNumber = Math.max(0, simulationNumber - 1);
        tableData = renderSimulationChart(
            svg as SVGElement,
            bounds,
            points,
            simulateSubgraph,
        );
    }

    $: if (svg) {
        let pointsToRender = points;
        if (smooth) {
            // Group points by label (simulation number)
            const groupedPoints = points.reduce(
                (acc, point) => {
                    acc[point.label] = acc[point.label] || [];
                    acc[point.label].push(point);
                    return acc;
                },
                {} as Record<number, Point[]>,
            );

            // Apply smoothing to each group separately
            pointsToRender = Object.values(groupedPoints).flatMap((group) => {
                const smoothedTimeCost = movingAverage(
                    group.map((p) => p.timeCost),
                    windowSize,
                );
                const smoothedCount = movingAverage(
                    group.map((p) => p.count),
                    windowSize,
                );
                const smoothedMemorized = movingAverage(
                    group.map((p) => p.memorized),
                    windowSize,
                );

                return group.map((p, i) => ({
                    ...p,
                    timeCost: smoothedTimeCost[i],
                    count: smoothedCount[i],
                    memorized: smoothedMemorized[i],
                }));
            });
        }

        tableData = renderSimulationChart(
            svg as SVGElement,
            bounds,
            pointsToRender,
            simulateSubgraph,
        );
    }
</script>

<div class="modal" class:show={shown} class:d-block={shown} tabindex="-1">
    <div class="modal-dialog modal-xl">
        <div class="modal-content">
            <ModalHeader bind:shown>
                {"Desired Retention Cost"}
            </ModalHeader>
            <div class="modal-body">
                <SimulatorConfig
                    bind:simulateFsrsRequest
                    bind:suspendLeeches
                    bind:leechThreshold
                    bind:newCardsIgnoreReviewLimit
                    bind:daysToSimulate
                    bind:deckSize
                    {openHelpModal}
                    {state}
                ></SimulatorConfig>
                <SwitchRow bind:value={smooth} defaultValue={true}>
                    <SettingTitle on:click={() => openHelpModal("simulateFsrsReview")}>
                        {"Smooth Graph"}
                    </SettingTitle>
                </SwitchRow>
                <button
                    class="btn {computing ? 'btn-warning' : 'btn-primary'}"
                    disabled={computing}
                    on:click={simulateFsrs}
                >
                    {tr.deckConfigSimulate()}
                </button>

                <button
                    class="btn {computing ? 'btn-warning' : 'btn-primary'}"
                    disabled={computing}
                    on:click={clearSimulation}
                >
                    {tr.deckConfigClearLastSimulate()}
                </button>

                <button
                    class="btn {computing ? 'btn-warning' : 'btn-primary'}"
                    disabled={computing}
                    on:click={() => {
                        $config.newPerDay = simulateFsrsRequest.newLimit;
                        $config.reviewsPerDay = simulateFsrsRequest.reviewLimit;
                        $config.maximumReviewInterval = simulateFsrsRequest.maxInterval;
                        $config.desiredRetention = simulateFsrsRequest.desiredRetention;
                        $newCardsIgnoreReviewLimit =
                            simulateFsrsRequest.newCardsIgnoreReviewLimit;
                        $config.reviewOrder = simulateFsrsRequest.reviewOrder;
                        $config.leechAction = suspendLeeches
                            ? DeckConfig_Config_LeechAction.SUSPEND
                            : DeckConfig_Config_LeechAction.TAG_ONLY;
                        $config.leechThreshold = leechThreshold;
                        $config.easyDaysPercentages = [
                            ...simulateFsrsRequest.easyDaysPercentages,
                        ];
                        onPresetChange();
                    }}
                >
                    <!-- {tr.deckConfigApplyChanges()} -->
                    {"Save to Preset Options"}
                </button>

                {#if simulating}
                    {tr.actionsProcessing()} {"Simulation"} {progress}/{20}
                {/if}

                <Graph>
                    <div class="radio-group">
                        <InputBox>
                            <label>
                                <input
                                    type="radio"
                                    value={SimulateSubgraph.count}
                                    bind:group={simulateSubgraph}
                                />
                                {tr.deckConfigFsrsSimulatorRadioCount()}
                            </label>
                            <label>
                                <input
                                    type="radio"
                                    value={SimulateSubgraph.time}
                                    bind:group={simulateSubgraph}
                                />
                                {tr.statisticsReviewsTimeCheckbox()}
                            </label>
                            <label>
                                <input
                                    type="radio"
                                    value={SimulateSubgraph.memorized}
                                    bind:group={simulateSubgraph}
                                />
                                {tr.deckConfigFsrsSimulatorRadioMemorized()}
                            </label>
                        </InputBox>
                    </div>

                    <div class="svg-container">
                        <svg
                            bind:this={svg}
                            viewBox={`0 0 ${bounds.width} ${bounds.height}`}
                        >
                            <CumulativeOverlay />
                            <HoverColumns />
                            <AxisTicks {bounds} />
                            <NoDataOverlay {bounds} />
                        </svg>
                    </div>

                    <TableData {tableData} />
                </Graph>
            </div>
        </div>
    </div>
</div>

<style>
    .modal {
        background-color: rgba(0, 0, 0, 0.5);
        --bs-modal-margin: 0;
    }

    .svg-container {
        width: 100%;
        max-height: calc(100vh - 400px); /* Account for modal header, controls, etc */
        aspect-ratio: 600 / 250;
        display: flex;
        align-items: center;
    }

    svg {
        width: 100%;
        height: 100%;
    }

    :global(.modal-xl) {
        max-width: 100vw;
    }

    div.radio-group {
        margin: 0.5em;
    }

    .btn {
        margin-bottom: 0.375rem;
    }
</style>
