# reltime

Relative time representations with language support.

## Features

- Expresses dates and times in natural, human-readable forms
- Supports relative (today, tomorrow), named (Monday, January), and exact representations
- Optional Swedish language support (enabled by default)
- JSON Schema generation via schemars
- Serialisation to natural JSON formats

## CLI

The CLI converts time expressions to their earliest or latest possible timestamps.

### Installation

```bash
cargo install --path cli
```

### Usage

Convert a time expression to its earliest timestamp:

```bash
reltime min today
reltime min monday
reltime min date 12 25  # 25th December this year
```

Convert to the latest timestamp:

```bash
reltime max today       # midnight tomorrow
reltime max this-week   # midnight next Monday
```

Specify a reference time:

```bash
reltime min --relative-to 2025-07-29T10:30:05Z today
```

Generate JSON Schema:

```bash
reltime schema > schema.json
```

## Schema Support

All types implement `JsonSchema` from the schemars crate. Generate a schema file for use with yaml-language-server or other schema-aware tools:

```bash
cd cli
cargo run -- schema > ../schema.json
```

## Example: Todo List with Schema

Create a `todo.yaml` with schema validation:

```yaml
# yaml-language-server: $schema=./schema.json
---
- task: Review pull requests
  due: Today

- task: Team meeting
  due: "14:30"

- task: Deploy to production
  due: Friday

- task: Q1 planning
  due: January
```

Another example with exact dates:

```yaml
# yaml-language-server: $schema=./schema.json
---
- task: Birthday present
  due: "25/12" # 25th December (recurring)

- task: Project deadline
  due: "15/3/2025 17:00" # 15th March 2025 at 5pm

- task: Morning standup
  due: "09:00"

- task: End of sprint
  due: ThisWeek
```

## Language Support

By default, Swedish variants are available alongside English:

```json
{
  "due": "Måndag"
}
```

To disable Swedish support, compile without default features:

```bash
cargo build --no-default-features
```
