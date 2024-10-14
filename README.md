![img.png](assets/img.png)

# meowlog
---

## WIP SUBJECT TO CHANGE

### ⚠️ IMPORTANT

The client's core functionality (managing substances and ingestions) works. The codebase is a mess and will be heavily
refactored so use at your own risk. There will be no backwards compatibility until the first stable release.

Planned features:

- [x] Managing ingestions and substances
- [ ] Having a sensible set of default substances (taken from TripSit or PsychonautWiki idk yet)
- [ ] Circular Concurrency Checking for binary files
- [ ] Server with syncing capabilities and maybe also a frontend with a similar feature set like the Psychonaut Wiki
  Journal app
- [ ] Referring to harm reduction resources in the CLI

Current problems:

- [ ] Codebase is a mess
- [ ] No tests
- [ ] Poor error handling
- [ ] Unoptimized memory usage

---

### Client usage:

```
Commands:
  add-ingestion     Adds ingestion
  edit-ingestion    Edits an ingestion
  list-ingestions   List ingestions
  remove-ingestion  Remove ingestion
  add-substance     Adds substance
  edit-substance    Edits an substance
  list-substances   List substances
  remove-substance  Remove substance
  help              Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

