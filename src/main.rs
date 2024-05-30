use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde_derive::Deserialize)]
pub struct Reserva{
    numero_reserva: i32,
    nome_hotel: String,
    numero_quarto: i32,
    data_check_in: String,
    data_check_out: String
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Hash{
    key: String,
    reserva: Reserva
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct HashMap{
    values: Vec<Hash>
}


impl Reserva{
    fn new(numero_reserva: i32, nome_hotel: String, numero_quarto: i32, data_check_in: String, data_check_out: String) -> Reserva{
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
        //Antes de inserir um novo valor, é verificado se já existe um hash na HASHMAP, caso exista o valor antigo é removido e o novo inserido
        let index: usize = hash.key[..2].parse().unwrap();
        if self.values.get(index) == None{
            self.values.push(hash);
        }else{
            self.remove(hash.key.clone());
            self.values.push(hash);
        }
        return;
    }

    fn display(&self){
        for (item, index) in self.values.iter().enumerate(){
            println!("OPÇÃO - {}\nHASH: {}\nRESERVA: {}\nHOTEL: {}\n", item, index.key, index.reserva.numero_reserva, index.reserva.nome_hotel);
        }
        return;
    }

    fn get(&self, target: usize){
        //Binary search para encontrar o valor da reserva
        let mut low = 0;
        let mut high = self.values.len() - 1;
        //let index: usize = target[..2].parse().unwrap();
        while low <= high {
            let mid = low + (high - low) / 2;
            if self.values[mid] == self.values[target]{
                //Se possivel incluir cores nos prints
                println!("Detalhes da Reserva {}:",target);
                println!("Chave: {}", self.values[mid].key);
                println!("Número da reserva: {}", self.values[mid].reserva.numero_reserva);
                println!("Nome do hotel: {}",self.values[mid].reserva.nome_hotel);
                println!("Número do quarto: {}",self.values[mid].reserva.numero_quarto);
                println!("Data do check-in: {}",self.values[mid].reserva.data_check_in);
                println!("Data do chech-out: {}",self.values[mid].reserva.data_check_out);
                return;
            } else if self.values[mid] < self.values[target]{
                low = mid + 1
            } else{
                high = mid - 1
            }
        }
        println!("Reserva não encontrada!");
        return;
    }

    fn remove(&mut self, target: String){
        //tratar o valor caso seja menor que 10
        let index: usize = target[..2].parse().unwrap();
        println!("Reserva removida!");
        self.values.remove(index);
        return;
    }

}


impl Hash{
    fn new(reserva: Reserva, size: i32) -> Hash{
        let nome_copy = &reserva.nome_hotel;
        let data_copy = &reserva.data_check_in;
        let key = Self::simple_hash(size, reserva.numero_reserva, reserva.numero_quarto,nome_copy.to_string(), data_copy.to_string());
        Hash{
            key,
            reserva
        }
    }
    fn simple_hash(size: i32,numero_reserva: i32, numero_quarto: i32, nome_hotel: String, data_check_in: String) -> String{   
        let hash_value = (((numero_reserva / nome_hotel.len() as i32) * numero_quarto) % 7) as i64;
        let mut hash: String = String::new();
        //Tratando o tamando do caracter inicial da hash, os valores possivel são 01 e 99.
        if size < 10{
            hash = "0".to_owned() + &size.to_string() + &data_check_in[..2].to_owned() + &hash_value.to_string() +  &nome_hotel.replace(" ", "")[..3].to_uppercase() +  &data_check_in[6..];
        } else{
            hash = size.to_string() + &data_check_in[..2].to_owned() + &hash_value.to_string() +  &nome_hotel.replace(" ", "")[..3].to_uppercase() +  &data_check_in[6..];
        }
        return hash;
    }
}

fn main() {
    let path = Path::new("./reservas.json");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let json: Vec<Reserva> = serde_json::from_reader(reader).unwrap();
    let mut hash_map: HashMap = HashMap::new();
    let mut input: String = String::new();
    let mut input2: String = String::new();
    for i in 0..=json.len()-1{
        let reserva: Reserva = Reserva::new(json[i].numero_reserva, json[i].nome_hotel.to_owned(), json[i].numero_quarto, json[i].data_check_in.to_owned(), json[i].data_check_out.to_owned());
        let hash: Hash = Hash::new(reserva, hash_map.values.len() as i32);
        hash_map.push(hash);
    }
    //Criar um loop até o usuário decidir parar
    //opções de incluir uma nova reserva, remover uma reserva e buscar uma reserva, cancelar uma reserva e realizar um check-out
    println!("Qual reserva você deseja ver mais informações:\n");
    hash_map.display();
    println!("\nEscolha entre as opções 0 a {}: ", hash_map.values.len()-1);

    stdin().read_line(&mut input).unwrap();
    let mut res: usize = input.trim().parse().unwrap();
    hash_map.get(res);

    println!("\nQual reserva você deseja remover: ");
    println!("\nEscolha entre as opções 0 a {}: ", hash_map.values.len()-1);

    stdin().read_line(&mut input2).unwrap();
    let res2: String = input2.trim().parse().unwrap();
    hash_map.remove(res2);
    hash_map.display();
}
