<!--
Copyright: Ankitects Pty Ltd and contributors
License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
-->
<script lang="ts">
    import AxisTicks from "./AxisTicks.svelte";
    import CumulativeOverlay from "./CumulativeOverlay.svelte";
    import { defaultGraphBounds } from "./graph-helpers";
    import type { HistogramData } from "./histogram-graph";
    import { histogramGraph } from "./histogram-graph";
    import HoverColumns from "./HoverColumns.svelte";
    import NoDataOverlay from "./NoDataOverlay.svelte";

    export let data: HistogramData | null = null;
    export let layers: number | null = null;

    const bounds = defaultGraphBounds();
    let svg = null as HTMLElement | SVGElement | null;

    $: histogramGraph(svg as SVGElement, bounds, data);
</script>

<svg bind:this={svg} viewBox={`0 0 ${bounds.width} ${bounds.height}`}>
    {#if layers == null}
        <g class="bars" />
    {:else}
        {#each Array(layers-1).map((_,i)=>(layers??1)-1-i) as layer}
            <g class="bars{layer}" />
        {/each}
    {/if}
    <HoverColumns />
    <CumulativeOverlay />
    <AxisTicks {bounds} />
    <NoDataOverlay {bounds} />
</svg>
