<!--
Copyright: Ankitects Pty Ltd and contributors
License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
-->
<script lang="ts">
    import type { SimulateFsrsReviewRequest } from "@generated/anki/scheduler_pb";
    import SpinBoxRow from "./SpinBoxRow.svelte";
    import SettingTitle from "$lib/components/SettingTitle.svelte";
    import SpinBoxFloatRow from "./SpinBoxFloatRow.svelte";
    import * as tr from "@generated/ftl";
    import EasyDaysInput from "./EasyDaysInput.svelte";
    import EnumSelectorRow from "$lib/components/EnumSelectorRow.svelte";
    import SwitchRow from "$lib/components/SwitchRow.svelte";
    import GlobalLabel from "./GlobalLabel.svelte";
    import { reviewOrderChoices } from "./choices";
    import type { DeckOptionsState } from "./lib";
    import { DeckConfig_Config_LeechAction } from "@generated/anki/deck_config_pb";
    import type { Writable } from "svelte/store";

    export let daysToSimulate;
    export let deckSize;
    export let suspendLeeches: boolean;
    export let newCardsIgnoreReviewLimit: Writable<boolean>;
    export let leechThreshold: number;

    export let simulateFsrsRequest: SimulateFsrsReviewRequest;
    export let state: DeckOptionsState;
    export let openHelpModal: (key: string) => void;

    $: config = state.currentConfig;
    $: fsrs = state.fsrs;
</script>

<SpinBoxRow bind:value={daysToSimulate} defaultValue={365} min={1} max={3650}>
    <SettingTitle on:click={() => openHelpModal("simulateFsrsReview")}>
        {tr.deckConfigDaysToSimulate()}
    </SettingTitle>
</SpinBoxRow>

<SpinBoxRow bind:value={deckSize} defaultValue={0} min={0} max={100000}>
    <SettingTitle on:click={() => openHelpModal("simulateFsrsReview")}>
        {tr.deckConfigAdditionalNewCardsToSimulate()}
    </SettingTitle>
</SpinBoxRow>

<SpinBoxFloatRow
    bind:value={simulateFsrsRequest.desiredRetention}
    defaultValue={$config.desiredRetention}
    min={0.7}
    max={0.99}
    percentage={true}
>
    <SettingTitle on:click={() => openHelpModal("desiredRetention")}>
        {tr.deckConfigDesiredRetention()}
    </SettingTitle>
</SpinBoxFloatRow>

<slot name="main"></slot>

<SpinBoxRow
    bind:value={simulateFsrsRequest.newLimit}
    defaultValue={$config.newPerDay}
    min={0}
    max={9999}
>
    <SettingTitle on:click={() => openHelpModal("simulateFsrsReview")}>
        {tr.schedulingNewCardsday()}
    </SettingTitle>
</SpinBoxRow>

<SpinBoxRow
    bind:value={simulateFsrsRequest.reviewLimit}
    defaultValue={$config.reviewsPerDay}
    min={0}
    max={9999}
>
    <SettingTitle on:click={() => openHelpModal("simulateFsrsReview")}>
        {tr.schedulingMaximumReviewsday()}
    </SettingTitle>
</SpinBoxRow>

<details>
    <summary>{tr.deckConfigEasyDaysTitle()}</summary>
    <EasyDaysInput bind:values={simulateFsrsRequest.easyDaysPercentages} />
</details>

<details>
    <summary>{"Advanced settings"}</summary>
    <SpinBoxRow
        bind:value={simulateFsrsRequest.maxInterval}
        defaultValue={$config.maximumReviewInterval}
        min={1}
        max={36500}
    >
        <SettingTitle on:click={() => openHelpModal("simulateFsrsReview")}>
            {tr.schedulingMaximumInterval()}
        </SettingTitle>
    </SpinBoxRow>

    <EnumSelectorRow
        bind:value={simulateFsrsRequest.reviewOrder}
        defaultValue={$config.reviewOrder}
        choices={reviewOrderChoices($fsrs)}
    >
        <SettingTitle on:click={() => openHelpModal("simulateFsrsReview")}>
            {tr.deckConfigReviewSortOrder()}
        </SettingTitle>
    </EnumSelectorRow>

    <SwitchRow
        bind:value={simulateFsrsRequest.newCardsIgnoreReviewLimit}
        defaultValue={$newCardsIgnoreReviewLimit}
    >
        <SettingTitle on:click={() => openHelpModal("simulateFsrsReview")}>
            <GlobalLabel title={tr.deckConfigNewCardsIgnoreReviewLimit()} />
        </SettingTitle>
    </SwitchRow>

    <SwitchRow
        bind:value={suspendLeeches}
        defaultValue={$config.leechAction == DeckConfig_Config_LeechAction.SUSPEND}
    >
        <SettingTitle on:click={() => openHelpModal("simulateFsrsReview")}>
            {"Suspend Leeches"}
        </SettingTitle>
    </SwitchRow>

    {#if suspendLeeches}
        <SpinBoxRow
            bind:value={leechThreshold}
            defaultValue={$config.leechThreshold}
            min={1}
            max={9999}
        >
            <SettingTitle on:click={() => openHelpModal("simulateFsrsReview")}>
                {tr.schedulingLeechThreshold()}
            </SettingTitle>
        </SpinBoxRow>
    {/if}
</details>

<style>
    summary {
        margin-bottom: 0.5em;
    }
</style>
