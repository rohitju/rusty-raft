@0xe7091badda8540a8;

struct VoteRequest {
    # Struct to represent a request for a vote

    candidateId @0 :Text; # The id of the candidate requesting the vote 
    term @1 :UInt32; # Term for which vote is being requested
    lastLogIndex @2 :UInt32; # Last log entry index for this candidate
    lastLogterm @3 :UInt32; # Term of last log index of candidate
}

struct VoteReply {
    term @0 :UInt32; # Current term of vote sender 
    granted @1 :Bool; # Has vote been granted?
}
