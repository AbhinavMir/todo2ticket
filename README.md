## Todo2ticket

work left, don't use yet
Export your todos are github/gitlab issues or tickets on jira/asana/linear/notion.

### Installation

@todo
```bash
brew install todo2ticket
```

### Usage

- Navigate to the directory where you want to run todo2tikcet.
- You can either run from a todo2ticket.yaml file or from command line
- You can either define your tickets in a ticket.json file or dump all tickets from repo into this file and then push it.
- Set your API keys as such (as needed): GITHUB_TOKEN, LINEAR_TOKEN, ASANA_TOKEN, NOTION_TOKEN, GITLAB_TOKEN, BITBUCKET_TOKEN.

**To run from CLI**

```bash
todo2ticket . # defaults to creating a ticket.json file from your current directory TODO comments
```

### Run from file

```bash
todo2ticket --path="~/IdeaProjects/sampleProject/.github/tickets/tickets.json" --config="~/IdeaProjects/sampleProject/.github/tickets/todo2ticket.yaml"
```

### Coming soon

1. Github Action template