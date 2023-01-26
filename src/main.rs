use parity_scale_codec::{Encode};
use parity_scale_codec_derive::{Encode};

#[derive(Debug, PartialEq, Encode)]
pub struct EquivocationProof {
	set_id: u64,
	equivocation: EquivocationEnum,
}

// Enum for prevote and precommit types
#[derive(Debug, PartialEq, Encode)]
pub enum EquivocationEnum {
	Prevote(Equivocation),
    Precommit(Equivocation),
}

// Prevote Equivocation
#[derive(Debug, PartialEq, Encode)]
pub struct Equivocation {
    pub round_number: u64,
    pub id: [u8; 32],
    pub first_hash: [u8; 32],
    pub first_num: u32,
    pub first_sig: [u8; 64],
    pub second_hash: [u8; 32],
    pub second_num: u32,
    pub second_sig: [u8; 64],
}

#[derive(Debug, PartialEq, Encode)]
pub struct Vote {
    pub hash: [u8; 32],
    pub number: u32,
}


fn main() {
    encode_equivocation();
    encode_vote();
}

fn encode_vote() {
    let block_hash: [u8; 32] = [10, 11, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let vote: Vote = Vote {  
        hash: block_hash, 
        number: 999, 
    };
    let enc =  vote.encode();
    println!("{:?}", enc);
}

fn encode_equivocation() {
    let round_num: u64 = 1;
    let block_num: u32 = 10;

    let authority_id: [u8; 32] = [136, 220, 52, 23, 213, 5, 142, 196, 180, 80, 62, 12, 18, 234, 26, 10, 137, 190, 32, 15, 233, 137, 34, 66, 61, 67, 52, 1, 79, 166, 176, 238];
    let first_block_hash: [u8; 32] = [72, 1, 184, 230, 45, 49, 22, 125, 48, 200, 147, 204, 25, 112, 246, 160, 226, 137, 66, 2, 130, 164, 178, 69, 183, 95, 44, 70, 251, 48, 138, 241];
    let second_block_hash: [u8; 32] = [195, 20, 50, 121, 65, 253, 217, 36, 188, 103, 253, 114, 101, 28, 64, 174, 206, 205, 72, 92, 163, 232, 120, 194, 30, 2, 171, 180, 15, 234, 229, 189];

    let first_vote_sig: [u8; 64] = [215, 41, 44, 170, 204, 98, 80, 67, 101, 241, 121, 137, 42, 115, 153, 242, 51, 148, 75, 242, 97, 248, 163, 246, 98, 96, 247, 14, 0, 22, 242, 219, 99, 146, 39, 38, 176, 21, 200, 45, 199, 19, 31, 71, 48, 251, 236, 97, 247, 22, 114, 165, 113, 69, 62, 81, 2, 155, 251, 70, 144, 112, 144, 15];
    let second_vote_sig: [u8; 64] = [179, 196, 8, 183, 73, 5, 223, 237, 255, 250, 102, 249, 159, 22, 254, 139, 147, 143, 216, 223, 118, 169, 34, 37, 34, 138, 28, 160, 117, 35, 11, 153, 162, 217, 225, 115, 197, 97, 149, 46, 30, 55, 139, 112, 25, 21, 202, 24, 141, 44, 131, 46, 249, 42, 63, 171, 142, 69, 95, 50, 87, 12, 8, 7];
   
    let equivocation: Equivocation = Equivocation { 
        round_number: round_num,
        id: authority_id, 
        first_hash: first_block_hash, 
        first_num: block_num, 
        first_sig: first_vote_sig, 
        second_hash: second_block_hash, 
        second_num: block_num, 
        second_sig: second_vote_sig 
    };

    let equivocation_enum = EquivocationEnum::Prevote(equivocation);

    let proof: EquivocationProof = EquivocationProof { 
        set_id: 1, 
        equivocation: equivocation_enum 
    };
    
    let enc =  proof.encode();
    println!("{:?}", enc);
}