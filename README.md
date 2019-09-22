# nmcm

This is a proof of concept pseudo-distributed key-value store as an exploration of a naive Paxos concensus-deciding algorithm.

Running this application will run through an example of a cache with a network partition:

```
Cache 1 created suggestion with id 1.
Cache 1 grants permission to accept suggestion id 1_1
Cache 2 grants permission to accept suggestion id 1_1
Cache 3 grants permission to accept suggestion id 1_1
Cache 1 has accepted suggestion id 1_1.
Cache 2 has accepted suggestion id 1_1.
Cache 3 has accepted suggestion id 1_1.
Cache 1 created suggestion with id 2.
Cache 1 grants permission to accept suggestion id 2_1
Cache 2 grants permission to accept suggestion id 2_1
Cache 3 grants permission to accept suggestion id 2_1
Cache 1 has accepted suggestion id 2_1.
Cache 2 has accepted suggestion id 2_1.
Cache 3 has accepted suggestion id 2_1.
Cache 1 online status set to false
Cache 1 was asked to create a suggestion, but it was offline.
Cache 2 created suggestion with id 3.
Cache 1 rejects request 3_2 due to being offline
Cache 2 grants permission to accept suggestion id 3_2
Cache 3 grants permission to accept suggestion id 3_2
Cache 1 rejects suggestion 3_2 due to being offline
Cache 2 has accepted suggestion id 3_2.
Cache 3 has accepted suggestion id 3_2.
Cache 1 was asked to create a suggestion, but it was offline.
Cache 2 created suggestion with id 4.
Cache 1 rejects request 4_2 due to being offline
Cache 2 grants permission to accept suggestion id 4_2
Cache 3 grants permission to accept suggestion id 4_2
Cache 1 rejects suggestion 4_2 due to being offline
Cache 2 has accepted suggestion id 4_2.
Cache 3 has accepted suggestion id 4_2.
Cache 1 online status set to true
Cache 1 created suggestion with id 3.
Cache 1 grants permission to accept suggestion id 3_1
Cache 2 rejects this suggestion, last accepted id is 4_2
Cache 3 rejects this suggestion, last accepted id is 4_2
Cache 2 created suggestion with id 5.
Cache 1 grants permission to accept suggestion id 5_2
Cache 2 grants permission to accept suggestion id 5_2
Cache 3 grants permission to accept suggestion id 5_2
Cache 1 has accepted suggestion id 5_2.
Cache 2 has accepted suggestion id 5_2.
Cache 3 has accepted suggestion id 5_2.
```