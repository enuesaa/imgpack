import Home from './routes/Home.svelte'
import NotFound from './routes/NotFound.svelte'
import Page from './routes/Page.svelte'

export default {
    '/': Home,
    '/page': Page,
    '*': NotFound,
}
