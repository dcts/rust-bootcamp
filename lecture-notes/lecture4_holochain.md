# Holochain Fundamentals
- google "fallacies of distributed computing"

# Personal Code / Coordinator Zomes (looku)
- API to interacti with the DHT space

# Integrity Zome (lookup)
- ges hashed to DNA

# Actions
- always public
- published to source chain AND the shared DHT space
- actions can attach entries (or not)

# Entries
- may be private or public
- represent data (post, etc..)

# Record
- Action + Entry = Record

# HDK 
- holochain development kit
- docs.rs/hdk
- for writing coordinator zomes

# HDI
- holochain deterministic integrity
- subset of HDK
- only for integrity code

# Holochain Conductor
- Holochain "Operating System" can run mutliple apps and holds cells

# Agent
- has an Agend ID
- Agent is basically a identity
- (TO CLAIRFY)one peson could have multiple Agents if they chose to

# Cell
- instance of a DNA

# DNA
- hApp integrity code (the part the is being hashes)

# Zome
- Modules to organize code / logic

# Links
- BaseHash => TargetHash
- Only in integrity zomes
- base, target, link type, tag

# Imortant Rust Code from HDK
```rust
// Fn     : agent_info() => 
// Fn     : zome_info() => 
// Fn     : create_entry() => 
// Struct : EntryType
// Struct : AgentPubKey
// Result : ExternResult
// Struct : ZomeName
// Struct : ZomeId
// Struct : DnaHash
// Struct : DnaInfo (?)

// Macro  : #[hdk_entry_defs]
// Macro  : #[hdk_entry_def]
// Macro  : #[hdk_entry_helper]
// Macro  : #[hdk_extern]
// Macro  : #[unit_enum(UnitEntryTypes)]
```

# Rust
```rust
// AgentPubKey::try_from(properties)? => returns result shorthand
```

# Entry Retrievals Get() => typo? comment_action_hash comment_hash
# 
