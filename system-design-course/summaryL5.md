# L5: The Many Types of Failures

1. split failures into categories
2. identify important categories, by the formula: frequency * impact
3. ideally monitor frequency and impact (otherwise guess)

### Failure Categories (examples)
- Network Failures: 
  - **TCP/IP** protocol to communicate (it can communicate, or it doesn't) 
  - **SSH** to enable privacy and encrypted information flows.
  - **connectivity** issues (no internet)
- Node Failures: 
  - **fail stop**: crash, power outage, hardware failure, out of memory
    - strategy1: checkpoint state and restart
    - strategy2: replicate and fail over (not sure what this is?)
  - **byzantine failure**: everything that can go wrong, that doesn't cause the node to stop!
    - node flips and corrupts data
    - software version incompatiility lead to wrong results, but note still runs
    - its hard to build a system that can tolerate byzantine failures => DON'T TRY TO ALLOW BYZANTINE FAILURES
- unclear: **network speed**: could be a network issue, or a node issue... 

### Network Partitioning
- when the graph looses a connecting node, the network might partition into multiple networks that cannot communicate anymore
- one possible strategy:
  - every node, ping all the other nodes in a given time interval
  - only write data to the system if you have N/2 or more connecting nodes! (50% at least)
  - that way you can ensure you are the biggest subnetwork
  - => consistency is enabled, but for the cost of accessibility!
  - if the network is often disconnected in multiple subgraphs, then the full network might come to a halt all together, because there is no majority system

# CAP Principle
You can have only 2 of those 3:
- C(onsistency): data is consistent, you cannot have 2 seperated networks writing to same database
- A(ccessibility): every node can use the program at all times, even if its only a subnetwork
- P(artitioning): the network is able to partition and heal itself when connection reestablish

