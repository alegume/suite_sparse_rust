# suite_sparse_rust


#### TODO
    testar perturbacao (hashmap e hashset)
    Implementar MILS
    refac print
    IMPLEMENTAR REGRAS  vizinhos_criticos!
    Inserir restart (grasp ideas?)
    Primeiro trocar todo mundo, depois trocar pela ideia do NCHC

    George Liu pseudo algo

#### DONE
    VErificar proposta com labels
    criar MILS crate

#### IDEIAS
    Swap
    Perturbação: Selecionar um u aleatorio para trocar com um vertice crítico. Manter um histórico de 100? vertices escolhidos para cada vertice v crítico
    TODO: lista de vértices ruim (maiores degrees do grafo)

    Shake-2 mladenovic? Talvez não
    TODO: (u, v)
    F(u) = G(u) + H(u) | G= é LB_min (degree(v)) ;  H = grau de v   |  Usar valor do grau de 2 niveis
    criar lista de vértice de menor grau 
    BFS = iniciar de um vértice de menor grau
    fazer a perturbacao usando essa lista
    busca local e repetir
        first improvement e best improvement
        multi partida, multi vizinhança (segmentação)
        


    Manter n soluções e depois utilizar path relinking

    BFS que verifica 2 níveis na frente (u -> v -> w)

    Na busca local, rotacionar vertices
    jogar o médio no meio
    u -> v -> w
    2 -> 7 -> 3
    a = |u - v|  - 5
    b = |v - w|  - 4
    c = |u - w|  - 1 *
    2, 3, 7 => 4
    min (
        min( a , b ),
        min( a, c ),
        min( c, b ),
    )