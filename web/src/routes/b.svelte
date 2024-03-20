<script>
  import { createCollapsible, melt } from '@melt-ui/svelte'

  const {
    elements: { root, content, trigger },
    states: { open }
  } = createCollapsible()
</script>

<div use:melt={$root}>
  <button use:melt={$trigger}>{$open ? 'Close' : 'Open'}</button>
  <div use:melt={$content}>Obi-Wan says: Hello there!</div>
</div>
