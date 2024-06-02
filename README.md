# Trabalho A3.2: Implementação de um Sistema de Reserva de Hotéis utilizando Hashmap

<h4 align="center"> 
	✅  Trabalho Finalizado
</h4>

# Informações do Grupo

UC: Estruturas de Dados e Análise de Algoritmos 

* José Mourão / RA: 121116938
* Gabriel Gontijo / RA: 121118394 
* Lucas Pina / RA: 121116933
* Diego Diniz / RA: 12119820
* Rafael Bastos / RA: 121119298

# Objetivos do Trabalho

A chave de cada reserva deverá ser gerada a partir de um número inteiro ou string Alpha Numérica que representa o número da reserva.

Cada reserva deverá conter os seguintes valores:

* Nome do Hotel ✅
* Numero do Quarto ✅ 
* Data de Check-In ✅
* Data de Check-Out ✅

O uso do hashmap deverá permitir:

* Inserir novas reservas de forma eficiente. ✅
* Recuperar detalhes de uma reserva rapidamente usando o código da reserva. ✅
* Remover reservas de forma eficiente após o check-out ou cancelamento ✅
* Uma função hash ORIGINAL deverá ser criada. As colisões deverão ser tratadas como possíveis reservas em caso de cancelamento, ou seja, se um reserva for cancelada a primeira colisão deverá assumir a reserva (defina bem sua estratégia de tratamento de colisão). ✅
* Implementação de mais uma função hash para comparação. ✅
* Deverá ser entregue um documento Readme.md com o descritivo do funcionamento da função. ✅

# Explicação da Primeira Função HASH usada

A primeira função que usamos é a do simple_hash. Ela recebe cinco parâmetros: "size", "numero_reserva", "numero_quarto", "nome_hotel" e "data_check_in". Com isso, ela cria e retorna uma string que representa um hash simplificado a partir desses parâmetros. 

O cálculo do "hash_value" é feito da seguinte forma: O parâmetro "numero_reserva" é dividido pelo tamanho do outro parâmetro, "nome_hotel", então o resultado disso é multiplicado por "numero_quarto". O produto que é conseguido será dividido por 7 e o resto dessa divisão será usado como o "hash_value". Por fim, ele é convertido para o tipo i64. 

Após isso, declaramos a a variável "hash" como uma string vazia, então o valor de "size" é checado; se for menor do que 10, o hash começará com 0 seguido pelo valor do "size", então será adicionado os primeiros dois caracteres do parâmetro "data_check_in", e o "hash_value" feito anteriormente será adicionado. Após espaços serem removidos e os caracteres minúsculos serem convertidos pra maiúsculos, os primeiros três caracteres do parâmetro "nome_hotel" também serão adicionados. Por fim, adiciona os caracteres do ano da "data_check_in" usada. 

Agora, se o valor de "size" for 10 ou maior, o hash começará com ele, e seguirá a mesma lógica explicada anteriormente, concatenando as partes de "data_check_in", "hash_value" e "nome_hotel". No final, a função retorna a string "hash" resultante. 

# Explicação da Segunda Função HASH usada

A segunda função que usamos é a do "hash_check_in". Ela recebe uma string representando uma data no formato "dd-mm-aaaa", então extrai os caracteres na seguinte ordem: Dia, mês e ano. Depois disso, é criado uma string nova que contém o valor de "mês", e os valores pegos de dia, mês e ano são convertidos para i32. Os valores são multiplicados, o módulo é dividido por 3 e é convertido para uma string. Por fim, os resultados são concatenados para formar a string final, usando o valor de mês, o de ano, o de dia e o da operação matemática, e com isso a string é retornada.

