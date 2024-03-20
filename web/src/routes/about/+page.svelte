<script>
  import { createCollapsible, melt } from '@melt-ui/svelte'
  import Header from '../../components/common/Header.svelte'
  const {
    elements: { root, content, trigger },
    states: { open }
  } = createCollapsible()
</script>

<Header title='aa' />
<div use:melt={$root}>
  <button use:melt={$trigger}>{$open ? 'Close' : 'Open'}</button>
  <div use:melt={$content}>Obi-Wan says: Hello there!</div>
</div>
