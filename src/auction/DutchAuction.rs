use crate::auction::Auction;

// Dutch action is the opposite of English auction, it’s also known
// as Open descending auction. The seller will initiate a high value
// and bidders will bid down from this high price, in other words, the
//  bid price will go down until a bidder’s willing to take it. Dutch
//  auction usually takes a short time to complete, if there’s more than
//   one item in bid, the bidding process will continue until the supply is equal to demand.
