# suite_sparse_rust


#### TODO
    revisar degree n cmr (só o que sai e o que entra?)
    Otimizar local_search (encontrar primeiro crítico/vizinhos e retornar)
    Revisar  vizinhos_criticos! 
    Inserir restart (grasp ideas?)
    Primeiro trocar todo mundo, depois trocar pela ideia do NCHC
    revisar GL com estratégia de nao visitados do CMr
    ! Retirar/revisar parte do grafo desconexo 
    vertices que não estao ligados a ninguem devem receber as rotulações por ultimo, não receber valores "do meio"

#### DONE
    George Liu pseudo algo
    IMPLEMENTAR REGRAS  vizinhos_criticos!
    refac print
    Implementar MILS
    Ajustar matrizes assimetricas
    Somente permitir m==n
    testar perturbacao (hashmap e hashset)
    testar bw_vertex revisar linha 119
    testar criticals
    VErificar proposta com labels
    criar MILS crate

#### IDEIAS
    Pq excentricidade 0? Vertices em que nenhum arco chega, apenas sai. Fica com 0 na distância. 
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