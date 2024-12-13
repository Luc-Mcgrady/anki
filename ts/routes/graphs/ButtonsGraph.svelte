<!--
Copyright: Ankitects Pty Ltd and contributors
License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
-->
<script lang="ts">
    import type { GraphsResponse } from "@generated/anki/stats_pb";
    import * as tr from "@generated/ftl";

    import AxisTicks from "./AxisTicks.svelte";
    import { ButtonGraphRange, renderButtons } from "./buttons";
    import Graph from "./Graph.svelte";
    import type { RevlogRange } from "./graph-helpers";
    import { defaultGraphBounds, GraphRange } from "./graph-helpers";
    import GraphRangeRadios from "./GraphRangeRadios.svelte";
    import HoverColumns from "./HoverColumns.svelte";
    import InputBox from "./InputBox.svelte";
    import NoDataOverlay from "./NoDataOverlay.svelte";

    export let sourceData: GraphsResponse | null = null;
    export let revlogRange: RevlogRange;

    let graphRange: ButtonGraphRange | GraphRange = GraphRange.Year;

    const bounds = defaultGraphBounds();

    let svg = null as HTMLElement | SVGElement | null;

    $: if (sourceData) {
        renderButtons(svg as SVGElement, bounds, sourceData, graphRange as ButtonGraphRange);
    }

    const title = tr.statisticsAnswerButtonsTitle();
    const subtitle = tr.statisticsAnswerButtonsSubtitle();
</script>

<Graph {title} {subtitle}>
    <InputBox>
        <label>
            <input type="radio" bind:group={graphRange} value={ButtonGraphRange.Today} />
            {tr.statisticsTrueRetentionToday()}
        </label>
        <label>
            <input type="radio" bind:group={graphRange} value={ButtonGraphRange.Yesterday} />
            {tr.statisticsTrueRetentionYesterday()}
        </label>
        <label>
            <input type="radio" bind:group={graphRange} value={ButtonGraphRange.Week} />
            {tr.statisticsTrueRetentionWeek()}
        </label>
        <GraphRangeRadios bind:graphRange={graphRange as GraphRange} {revlogRange} followRevlog={true} />
    </InputBox>

    <svg bind:this={svg} viewBox={`0 0 ${bounds.width} ${bounds.height}`}>
        <g class="bars" />
        <HoverColumns />
        <AxisTicks {bounds} />
        <NoDataOverlay {bounds} />
    </svg>
</Graph>
