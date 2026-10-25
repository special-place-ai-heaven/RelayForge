# Inventory implementation results

M005-S01 adds strict version-1 inventory and request JSON, immutable SQLite
snapshot import, exact-scope capability eligibility, and explicit CLI
diagnostics. Snapshots remain declared metadata. A library caller must supply a
separate `CurrentState`; the API documentation says that callers own trustworthy
capture and that matching fields do not authenticate it.

## CLI

The explicit-path commands are:

```text
relayforge inventory-import --db <db> --input <snapshot.json>
relayforge inventory-show --db <db> --snapshot <id>
relayforge eligibility --db <db> --snapshot <id> --input <request.json>
```

Fresh-process integration coverage imported and replayed a snapshot, read it by
ID, and ran eligibility diagnostics. The standalone command returned
`binding_unavailable`, `no_eligible_candidate`, and `missing_current_state`;
it does not turn JSON assertions into runtime confirmation.

## Limits

Focused verification exercised inventory schema/privacy/size checks, immutable
replay and conflict behavior, retained pre-existing event rows, current-state
matching, capability statuses, future/expired evidence, selection, dirty
workspaces, stable ordering, and fresh-process CLI behavior. The tests use
synthetic metadata. No live runtime observation, G4 host binding, ranking,
provider call, or dispatch was exercised or claimed.
