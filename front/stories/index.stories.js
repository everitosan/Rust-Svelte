import Title from '../src/Components/Title.svelte'

export default { title: "Title"}

export const textUsage = () => ({
    Component: Title,
    props: {
        text: "Ejemplo de componente Title"
    }
})