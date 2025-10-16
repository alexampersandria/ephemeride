<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Entry from '$lib/assemblies/Entry.svelte'
import { defaultCategories } from '$lib/types/log'
</script>

# Entry

A comprehensive journal entry component that supports viewing, editing, and creating entries with mood tracking, markdown text, and categorized tags.

## Usage

The Entry assembly is the main component for managing journal entries. It provides three modes (view, edit, create) and integrates mood input, markdown-formatted text, and a flexible category/tag system.

### Basic Usage

<DocsExample>
  <Entry categories={defaultCategories} />
</DocsExample>

```svelte
<script>
import { defaultCategories } from '$lib/types/log'
</script>

<Entry categories={defaultCategories} />
```

### Create

`create` mode is used for creating new entries. It provides a clean interface with all input fields available. And changes to view upon saving, where you can then enter edit mode.

<DocsExample>
  <Entry 
    mode="create"
    categories={defaultCategories} 
  />
</DocsExample>

```svelte
<Entry mode="create" categories={defaultCategories} />
```

## Types

### Props

| Name           | Type                           | Required | Default         | Description                                                                    |
| -------------- | ------------------------------ | :------: | --------------- | ------------------------------------------------------------------------------ |
| date           | `string`                       |          | `currentDate()` | Date string in YYYY-MM-DD format. Displayed as the entry timestamp.            |
| mode           | `'view' \| 'edit' \| 'create'` |          | `'view'`        | Current mode of the entry component. Controls editing capabilities.            |
| categories     | `CategoryWithTags[]`           |          | `[]`            | Array of categories with their associated tags.                                |
| entry          | `string`                       |          | `''`            | The entry text content. Supports markdown formatting.                          |
| mood           | `MoodValue`                    |          | `undefined`     | Mood value (1-5). Optional mood rating for the entry.                          |
| selectedTagIds | `string[]`                     |          | `[]`            | Array of selected tag IDs. Used to track which tags are active for this entry. |

## References

- [MoodInput](/docs/components/moodinput)
- [Category](/docs/assemblies/category)
