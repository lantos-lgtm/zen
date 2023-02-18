std: std()
{
    Element,
    Div,
}: std.web.elements
MyStore: std.package.MyStore


DarkMode: Element {
    type: Div
    props: {
        className: 'dark-mode'
    }
    children: {
        input: {
            type: 'input'
            props: {
                type: 'checkbox'
                checked: MyStore.isDarkMode
                onChange: {
                    MyStore.isDarkMode = !MyStore.isDarkMode
                }
            }
        }
        label: {
            type: 'label'
            props: {
                htmlFor: 'dark-mode'
            }
            children: 'Dark Mode'
        }
    }
    onRender: {
        children.input.addEventListener('change', () => {
            MyStore.isDarkMode = !MyStore.isDarkMode
        })
    }
}
