<script lang="ts">
    let zoom_value = 100;

    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMount } from "svelte";
    import { writable } from "svelte/store";

    let notepad: HTMLElement = null;
    const statusbar = writable(true);
    onMount(async () => {
        const textarea = notepad as HTMLInputElement;
        await getCurrentWindow().listen("status_bar", (event) => {
            $statusbar = event.payload as boolean;
        });
        await getCurrentWindow().listen("insert_char", (event) => {
            let currentPos = textarea.selectionStart;
            let endPos = textarea.selectionEnd;
            let content = textarea.value;

            // insert char at cursor pos
            textarea.value = `${content.substring(0, currentPos)}${event.payload}${content.substring(endPos, content.length)}`;
            // move cursor right
            textarea.setSelectionRange(currentPos + 1, currentPos + 1);
        })
        await getCurrentWindow().listen("backspace", (event) => {
            let currentPos = textarea.selectionStart;
            if (currentPos === 0) return;
            let content = textarea.value;

            textarea.value = content.substring(0, currentPos - 1) + content.substring(currentPos);
            textarea.setSelectionRange(currentPos - 1, currentPos - 1);
        })
        await getCurrentWindow().listen("cursor", (event) => {
            let dir = event.payload as number;
            let currentPos = textarea.selectionStart;
            textarea.setSelectionRange(currentPos + dir, currentPos + dir);
        })
     })
</script>
<main>
  <textarea bind:this={notepad} id="notepad" on:focus|preventDefault on:click|preventDefault on:input|preventDefault on:keydown|preventDefault></textarea>
  {#if $statusbar}
    <div id="status-bar">
        <div id="file-info">
            <span id="line-info">Ln 1, Col 1</span>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <span id="zoom">{zoom_value}%</span>
            <span id="linefeed">Windows (CLRF)</span>
            <span id="charset">UTF-8</span>
        </div>
    </div>
  {/if}
</main>

<style lang="scss">
  main {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }
  #notepad {
    border: none;
    flex: 1;
    resize: none;
    overflow: scroll;
    &:focus {
      outline: none;
    }
    scrollbar-color: #d7d7d7 #F0F0F0;
  }
    #status-bar {
        min-height: 22px;
        width: 100%;
        background-color: #F0F0F0;
        box-shadow: 0px -1px 0px #d7d7d7;
        position: relative;
        display: flex;
        align-items: center;
        font-size: 12px;
        cursor: default;
    }
    #file-info {
        display: flex;
        justify-content: flex-end;
        flex-grow: 1;
        height: 20px;
        span {
            padding: 0 10px;
            height: 100%;
            display: flex;
            justify-content: flex-start;
            align-items: center;
            &:hover {
                cursor: pointer;
            }
            &:first-child {
              border-left: 1px solid #d7d7d7;
            }
            &:not(:last-child) {
              border-right: 1px solid #d7d7d7;
            }
            &:not(#zoom) {
              min-width: 100px;
            }
        }
    }
</style>
