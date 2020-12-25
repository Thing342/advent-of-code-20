type Int = i64;

#[derive(Debug,Copy, Clone)]
struct PublicKey(Int);
#[derive(Debug,Copy, Clone)]
struct EncryptionKey(Int);

const PSK: Int = 20201227;

const CARD_KEY: PublicKey = PublicKey(16915772);
const DOOR_KEY: PublicKey = PublicKey(18447943);
//const CARD_KEY: PublicKey = PublicKey(5764801);
//const DOOR_KEY: PublicKey = PublicKey(17807724);

const CARD_SN: Int = 7;
const DOOR_SN: Int = 7;

#[derive(Debug)]
struct Device {
    key: PublicKey,
    sn: Int,
    ln: Int
}

impl Device {
    fn from_key(key: PublicKey, sn: Int) -> Self {
        let mut a_n = 1;
        let mut ln = 0;
        while a_n != key.0 {
            a_n = (sn * a_n) % PSK;
            ln += 1;
        }
        Device {
            key, sn, ln
        }
    }

    fn encryption_key(&self, other_sn: Int) -> EncryptionKey {
        let mut a_n = 1;
        for _ in 0..self.ln {
            a_n = (other_sn * a_n) % PSK;
        }
        EncryptionKey(a_n)
    }
}


pub fn main() {
    let card = Device::from_key(CARD_KEY, CARD_SN);
    //let door = Device::from_key(DOOR_KEY, DOOR_SN);
    eprintln!("card: {:?}", &card);

    let key = card.encryption_key(DOOR_KEY.0);
    eprintln!("DAY 25, PART 1: {}", key.0);
}