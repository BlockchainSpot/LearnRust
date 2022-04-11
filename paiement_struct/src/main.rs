
enum Payment {
    Cash(f32),
    CreditCard(String, f32),
    DebitCard(DebitData),
    Crypto{accound_id: String, amount: f32},
}

struct DebitData {
    pub card_number: String,
    pub amount: f32,
}

fn main() {

    let some_payment = Payment::Cash(100.);
    process_payment(some_payment);

    let cc_payment = Payment::CreditCard("CC Num".to_string(), 250.);
    process_payment(cc_payment);

    let debit_payment = Payment::DebitCard(DebitData {
        card_number: "Debit num".to_string(),
        amount: 400.,
    });
    process_payment(debit_payment);

    let crypto_payment = Payment::Crypto{accound_id: "0c.....".to_string(), amount: 1000.};
    process_payment(crypto_payment);
  
}

fn process_payment(some_payment: Payment) {

     match some_payment {
        Payment::Cash(amt) => {
            println!("Paying with cash... in the amout of {}", amt);
        }
        Payment::CreditCard(_some_string, _some_f32)=> {
            println!("Paying with credit card... some_String {}, some_f32 {}", _some_string, _some_f32);
        }
        Payment::DebitCard(_data) => {
            println!("Paying withe debit... card_number {}, amout{}", _data.card_number, _data.amount);
        }
        Payment::Crypto{accound_id, amount} => {
            println!("Paying with crypto... amount_id {}, amount {}", accound_id, amount);
        }
    }
}