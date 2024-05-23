Para este projeto A3.2, deveremos construir um sistema de reservas de hotéis, onde a eficiência na busca, inserção e remoção de reservas é crucial. O hashmap pode ser usado para mapear o número de identificação da reserva (como chave) para os detalhes da reserva (como valor).

 

A chave de cada reserva deverá ser gerada a partir de um número inteiro ou string Alpha Numérica que representa o número da reserva.

 

Cada reserva deverá conter os seguintes valores:

Nome do Hotel
Numero do Quarto
Data de Check-In
Data de Check-Out
O uso do hashmap deverá permitir:

Inserir novas reservas de forma eficiente.
Recuperar detalhes de uma reserva rapidamente usando o código da reserva.
Remover reservas de forma eficiente após o check-out ou cancelamento
Uma função hash ORIGINAL deverá ser criada. As colisões deverão ser tratadas como possíveis reservas em caso de cancelamento, ou seja, se um reserva for cancelada a primeira colisão deverá assumir a reserva (defina bem sua estratégia de tratamento de colisão).

O projeto deverá ser entregue através do ULife em pacote único.

 

Deverá ser entregue um documento Readme.md com o descritivo do funcionamento da função. 

 

O trabalho poderá ser feito em grupos de até 6 pessoas, com as seguintes condições:

 

Individual ou Dupla	Somente entrega pelo ULife
Trio ou Quarteto	Somente entrega pelo ULife, porém com a implementação de mais uma função hash para comparação.
Mais que 4 alunos	Entrega pelo ULife até a data do trabalho e apresentação para o professor do trabalho em sala de aula, além da implementação de mais uma função hash para comparação.
 

Deverá ser implantada um carga de dados via arquivo para testes em massa da solução, além disso, o sistema deve dar a saída de como a hashmap ficou configurada (podendo ser na tela ou em arquivo).

 

O formato do arquivo txt para carga será o seguinte:


Onde a primeira linha representa o número de posições da hash e todas as próximas representam um registro a ser inserido na hash. Podem ser N inserções, o exemplo acima mostra 4.

 

O Desenvolvimento pode ser feito em Java, Rust, C++ ou C#

 

Bom trabalho.