#include <iostream>
#include <unordered_map>
#include <unordered_set>

using namespace std;


class Graph {
    private:
        unordered_map<string, unordered_set<string> > adjList;
    
    public:
        void printGraph() {
            unordered_map<string, unordered_set<string>>::iterator kvPair = adjList.begin();
            while (kvPair != adjList.end()) {
                cout  << kvPair->first << ": [ ";  // this prints out the vertex
                unordered_set<string>::iterator edge = kvPair->second.begin();
                while (edge != kvPair->second.end()) {
                    cout  << edge->data() << " ";  // this prints out edges
                    edge++;
                }
                cout << "]" << endl;
                kvPair++;
            }
        }

        bool addVertex(string vertex) {
            if (adjList.count(vertex) == 0) {
                adjList[vertex];
                return true;
            }
            return false;
        }

        bool addEdge(string vertex1, string vertex2) {
            if (adjList.count(vertex1) != 0 && adjList.count(vertex2) != 0) {
                adjList.at(vertex1).insert(vertex2);
                adjList.at(vertex2).insert(vertex1);
                return true;
            }         
            return false;            
        }

        bool removeEdge(string vertex1, string vertex2) {
            if (adjList.count(vertex1) != 0 && adjList.count(vertex2) != 0) {
                adjList.at(vertex1).erase(vertex2);
                adjList.at(vertex2).erase(vertex1);
                return true;
            }
            return false;
        }

        bool removeVertex(string vertex) { 
            if (adjList.count(vertex) == 0) return false;  
            for (auto otherVertex : adjList.at(vertex)) {
                adjList.at(otherVertex).erase(vertex);   
            }
            adjList.erase(vertex);
            return true;
        }

};


int main() {

    Graph* myGraph = new Graph();

    myGraph->addVertex("A");
    myGraph->addVertex("B");
    myGraph->addVertex("C");
    myGraph->addVertex("D");

    myGraph->addEdge("A", "B");
    myGraph->addEdge("A", "C");
    myGraph->addEdge("A", "D");
    myGraph->addEdge("B", "D");
    myGraph->addEdge("C", "D");


    cout << "Graph before removeVertex(): \n";
    myGraph->printGraph();


    myGraph->removeVertex("D");


    cout << "\n\nGraph after removeVertex(): \n";
    myGraph->printGraph();
    
    /*
        EXPECTED OUTPUT:
        ----------------
        Graph before removeVertex(): 
        D: [ A B C ]
        C: [ A D ]
        B: [ A D ]
        A: [ B C D ]


        Graph after removeVertex(): 
        C: [ A ]
        B: [ A ]
        A: [ B C ]

    */ 
   
}

