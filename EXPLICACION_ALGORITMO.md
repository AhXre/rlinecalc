## Caso simple, para una operación sin parentesis
```
[5, +, 8, -, 3]

la deque se crea así:
[5] <= (push_back)
[+,5] <= (push_front)
[+,5,8] <= (push_back)
[-,+,5,8] <= (push_front)
[-,+,5,8,3] <= (push_back)

la lista resultante, es la lectura de un arbol en profundidad, el cual el número de hijos es definido por el operador, en este caso, todos los operadores tienen 2 hijos, entonces, el arbol se reconstruiria de la siguiente forma

      (-)
     /   \
   (+)   (3)
  /   \
(5)   (8)

y las operaciones se ejecutan de el mismo modo de profundidad, por lo que cuando lee (-), espera ver dos entradas, pero si la entrada es una operación, lo que hace es primero resolver la operación y así, recursivamente

```


## Caso con parentesis
```
[5,+,(,8,-,3,)]

la lista se crea así 

lista: [5] <= (push_back)
lista: [+,5] <= (push_front)
(se hace recursión creando una nueva sublista lista_1 y al final se añade como si fuera un valor, es decir push_back)
lista_1: [8] <= (push_back)
lista_1: [-,8] <= (push_front)
lista_1: [-,8,3] <= (push_back)
lista: [+,5,-,8,3] (se lee el final de parentesis y se cierra la recursión)

Para procesar los parentesis, se necesita hacer una recursión y crear una nueva lista para después poder añadirla a la lista padre por así decirlo, de esta forma se tendría el arbol:

      (+)
     /   \
   (5)   (-)
        /   \            
      (8)   (3)
```
## Caso con jerarquia de operadores

```
[5,+,8,*,9,/,3,-,1]

la lista se crea así:

lista: [5] <= (push_back)
lista: [+,5] <= (push_front)
lista: [+,5,8] <= (push_back)
(en este caso se lee *, por lo que se recoge el ultimo valor, se hace pop_back y se hace recursión pero por prioridad, es decir, cuando se lea el proximo operador de un nivel de jerarquia inferior, entonces, se devolverá en la recursión, y se continuará)
lista_1: [*,8]
lista_1: [*,8,9] <= (push_back)
lista_1: [/,*,8,9] <= (push_front)
lista_1: [/,*,8,9,3] <= (push_back)
(ahora lee (en forma de peek) -, mira que es menor, entonces, se devuelve en la recursión, añade lo que tenia en forma de push_back)
lista: [+,5,/,*,8,9,3]
lista: [-,+,5,/,*,8,9,3] <= (push_back)
lista: [-,+,5,/,*,8,9,3,1] <= (push_front)

Se tiene como resultado el arbol:
      (-)
     /   \
   (+)   (1)
  /   \     
(5)   (/) 
     /   \
   (*)   (3)
  /   \
(8)   (9)
```
