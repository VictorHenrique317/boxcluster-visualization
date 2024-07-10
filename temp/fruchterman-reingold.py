import networkx as nx
import matplotlib.pyplot as plt
import math
from random import random
from numpy import arange

N = 1000
K = 5
P = 0.05

# attractive force
def f_a(d,k):
    return d*d/k

# repulsive force
def f_r(d,k):
    return k*k/d

def fruchterman_reingold(G,iteration=50):
    W = 1
    L = 1
    area = W*L
    k = math.sqrt(area/nx.number_of_nodes(G))

    # initial position
    for v in nx.nodes(G):
        G._node[v]['x'] = W*random()
        G._node[v]['y'] = L*random()


    t = W/10
    dt = t/(iteration+1)

    print("area:{0}".format(area))
    print("k:{0}".format(k))
    print("t:{0}, dt:{1}".format(t,dt))

    for i in range(iteration):
        print("iter {0}".format(i))

        pos = {}
        for v in G.nodes():
            pos[v] = [G._node[v]['x'],G._node[v]['y']]
        plt.close()
        plt.ylim([-0.1,1.1])
        plt.xlim([-0.1,1.1])
        plt.axis('off')
        nx.draw_networkx(G,pos=pos,node_size=10,width=0.1,with_labels=False)
        plt.savefig("fig/{0}.png".format(i))

        # calculate repulsive forces
        for v in G.nodes():
            G._node[v]['dx'] = 0
            G._node[v]['dy'] = 0
            for u in G.nodes():
                if v != u:
                    dx = G._node[v]['x'] - G._node[u]['x']
                    dy = G._node[v]['y'] - G._node[u]['y']
                    delta = math.sqrt(dx*dx+dy*dy)
                    if delta != 0:
                        d = f_r(delta,k)/delta
                        G._node[v]['dx'] += dx*d
                        G._node[v]['dy'] += dy*d

        # calculate attractive forces
        for v,u in G.edges():
            dx = G._node[v]['x'] - G._node[u]['x']
            dy = G._node[v]['y'] - G._node[u]['y']
            delta = math.sqrt(dx*dx+dy*dy)
            if delta != 0:
                d = f_a(delta,k)/delta
                ddx = dx*d
                ddy = dy*d
                G._node[v]['dx'] += -ddx
                G._node[u]['dx'] += +ddx
                G._node[v]['dy'] += -ddy
                G._node[u]['dy'] += +ddy

        # limit the maximum displacement to the temperature t
        # and then prevent from being displace outside frame
        for v in G.nodes():
            dx = G._node[v]['dx']
            dy = G._node[v]['dy']
            disp = math.sqrt(dx*dx+dy*dy)
            if disp != 0:
                cnt += 1
                d = min(disp,t)/disp
                x = G._node[v]['x'] + dx*d
                y = G._node[v]['y'] + dy*d
                x =  min(W,max(0,x)) - W/2
                y =  min(L,max(0,y)) - L/2
                G.node[v]['x'] = min(math.sqrt(W*W/4-y*y),max(-math.sqrt(W*W/4-y*y),x)) + W/2
                G.node[v]['y'] = min(math.sqrt(L*L/4-x*x),max(-math.sqrt(L*L/4-x*x),y)) + L/2

        # cooling
        t -= dt

    pos = {}
    for v in G.nodes_iter():
        pos[v] = [G.node[v]['x'],G.node[v]['y']]
    plt.close()
    plt.ylim([-0.1,1.1])
    plt.xlim([-0.1,1.1])
    plt.axis('off')
    nx.draw_networkx(G,pos=pos,node_size=10,width=0.1,with_labels=False)
    plt.savefig("fig/{0}.png".format(i+1))

    return pos

def main():
    G = nx.watts_strogatz_graph(N,K,P)
    pos = fruchterman_reingold(G)

    plt.close()
    plt.ylim([-0.1,1.1])
    plt.xlim([-0.1,1.1])
    plt.axis('off')
    nx.draw_networkx(G,pos=nx.spring_layout(G),node_size=10,width=0.1,with_labels=False)
    plt.savefig("fig/orig.png")

if __name__ == "__main__":
    main()