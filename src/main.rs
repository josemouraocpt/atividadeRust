struct Reserva{
    numero_reserva: i32,
    nome_hotel: String,
    numero_quarto: i16,
    data_check_in: String,
    data_check_out: String
}

struct Hash{
    key: f32,
    reserva: Reserva
}

struct HashMap{
    values: Vec<Hash>
}


impl Reserva{
    fn new(numero_reserva: i32, nome_hotel: String, numero_quarto: i16, data_check_in: String, data_check_out: String) -> Reserva{
        Reserva{
            numero_reserva,
            nome_hotel,
            numero_quarto,
            data_check_in,
            data_check_out
        }
    }
}

impl HashMap{
    fn new() -> HashMap{
        HashMap{
            values: Vec::new()
        }
    }

    fn push(&mut self, hash: Hash){
        self.values.push(hash)
    }

    fn display(&self){
        for item in &self.values{
            println!("HASH: {}", item.key);
            //println!("RESERVA: {}", item.reserva)
        }
    }

}

impl Hash{
    fn new(reserva: Reserva) -> Hash{
        let key: f32 = simple_hash(reserva.numero_reserva);
        Hash{
            key,
            reserva
        }
    }

    fn simple_hash(numero_reserva: i32) -> f32{
        //em caso de colisão remover o primeiro caso
        let hash_value: f32 = (numero_reserva % 7) as f32;
        return hash_value;
    }
}
fn main() {
    let mut hash_map: HashMap = HashMap::new();
    let reserva1: Reserva = Reserva::new(11, "Nome do hotel".to_owned(), 10, "data 1".to_owned(), "data 2".to_owned());
    let reserva2: Reserva = Reserva::new(1, "Nome do hotel".to_owned(), 10, "data 1".to_owned(), "data 2".to_owned());
    let reserva3: Reserva = Reserva::new(5, "Nome do hotel".to_owned(), 10, "data 1".to_owned(), "data 2".to_owned());
    let reserva4: Reserva = Reserva::new(3, "Nome do hotel".to_owned(), 10, "data 1".to_owned(), "data 2".to_owned());
    let reserva5: Reserva = Reserva::new(99, "Nome do hotel".to_owned(), 10, "data 1".to_owned(), "data 2".to_owned());
    let hash1: Hash = Hash::new(reserva1);
    let hash2: Hash = Hash::new(reserva2);
    let hash3: Hash = Hash::new(reserva3);
    let hash4: Hash = Hash::new(reserva4);
    let hash5: Hash = Hash::new(reserva5);
    hash_map.push(hash1);
    hash_map.push(hash2);
    hash_map.push(hash3);
    hash_map.push(hash4);
    hash_map.push(hash5);
    hash_map.display()

    
}
