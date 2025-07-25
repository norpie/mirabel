<script lang="ts">
    import * as Resizable from '$lib/components/ui/resizable/index.js';
    import Spinner from '$lib/components/spinner.svelte';
    import ChevronsRight from 'lucide-svelte/icons/chevrons-right';
    import ChevronsLeft from 'lucide-svelte/icons/chevrons-left';

    import Chat from './chat.svelte';
    import Monitor from './monitor.svelte';

    import type { PaneAPI } from 'paneforge';
    import type { PageProps } from './$types';

    import { selectedWorkspace, selectedSession, sessions } from '$lib/store';
    import type { SocketHandler } from '$lib/socket.svelte';
    import { SessionState } from '$lib/session-state.svelte';
    import type { TimelineEntry, UserInteraction } from '$lib/models/session';

    let { data }: PageProps = $props();

    let sessionState = $state(new SessionState(data.user, data.session, data.socket));

    let chatPane: PaneAPI | undefined = $state();
    let monitorPane: PaneAPI | undefined = $state();

    let inset: HTMLDivElement | undefined = $state();

    let socket: SocketHandler<TimelineEntry, UserInteraction> | undefined = $state(data.socket);
    let session = $state(data.session);

    const minSize = 5;
    const maxSize = 100 - minSize;
    const hideSize = 15;
    const chatSize = 35;
    const monitorSize = 100 - chatSize;
    const smallScreenSize = 768;

    let isSmallScreen = $state(false);
    let disableResize = $derived(isSmallScreen);
    let enableHandle = $derived(!disableResize);

    let tab = $state('plan');

    function handleResize() {
        if (!inset) return;
        if (inset?.clientWidth < smallScreenSize && !isSmallScreen) {
            isSmallScreen = true;
            chatPane?.resize(maxSize);
            monitorPane?.resize(minSize);
        } else if (inset.clientWidth >= smallScreenSize && isSmallScreen) {
            isSmallScreen = false;
            chatPane?.resize(chatSize);
            monitorPane?.resize(monitorSize);
        }
    }

    function switchSide() {
        if (isSmallScreen) {
            if (chatPane?.getSize() == maxSize) {
                chatPane?.resize(minSize);
                monitorPane?.resize(maxSize);
            } else {
                chatPane?.resize(maxSize);
                monitorPane?.resize(minSize);
            }
        } else {
            chatPane?.resize(chatSize);
            monitorPane?.resize(monitorSize);
        }
    }

    $effect(() => {
        selectedWorkspace.set(data.workspace);
        selectedSession.set(data.session);
        sessions.set(data.sessions);
    });

    $effect(() => {
        handleResize();
        window.addEventListener('resize', handleResize);
        return () => {
            window.removeEventListener('resize', handleResize);
        };
    });

    // Socket connection management - persistent across component unmounting
    $effect(() => {
        if (socket && session) {
            console.log('Connecting socket for session:', session.id);
            socket.connect();
        }

        return () => {
            if (socket) {
                console.log('Disconnecting socket for session:', session.id);
                socket.disconnect();
            }
        };
    });

    // Ensure sessionState stays connected to socket even when chat unmounts
    $effect(() => {
        if (socket && sessionState) {
            sessionState.socket = socket;
        }
    });
</script>

<div bind:this={inset} class="h-full rounded-xl bg-primary md:min-h-min">
    {#if session}
        <Resizable.PaneGroup direction="horizontal" class="h-full">
            <Resizable.Pane
                id="chat"
                class="flex h-full flex-col"
                bind:this={chatPane}
                defaultSize={chatSize}
                {minSize}
            >
                {#if chatPane?.getSize() < hideSize}
                    <button
                        class="flex h-full w-full items-center justify-center rounded-l-xl transition-colors hover:bg-secondary"
                        onclick={() => switchSide()}
                    >
                        <ChevronsRight />
                    </button>
                {:else}
                    <Chat bind:sessionState />
                {/if}
            </Resizable.Pane>
            <Resizable.Handle withHandle={enableHandle} disabled={disableResize} />
            <Resizable.Pane
                bind:this={monitorPane}
                defaultSize={monitorSize}
                {minSize}
                class="flex h-full flex-col"
            >
                {#if monitorPane?.getSize() < hideSize}
                    <button
                        class="flex h-full w-full items-center justify-center rounded-r-xl transition-colors hover:bg-secondary"
                        onclick={() => switchSide()}
                    >
                        <ChevronsLeft />
                    </button>
                {:else}
                    <Monitor bind:tab bind:sessionState />
                {/if}
            </Resizable.Pane>
        </Resizable.PaneGroup>
    {:else}
        <div class="flex h-full w-full items-center justify-center">
            <Spinner />
        </div>
    {/if}
</div>
