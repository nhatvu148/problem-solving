#include <iostream>
#include <queue>

using namespace std;


class Node { 
    public: 
        int value;
        Node* left;
        Node* right;

    
        Node(int value) {
            this->value = value;
            left = nullptr;
            right = nullptr;
        }
};


class BinarySearchTree {
    public:
        Node* root;

    public:
        BinarySearchTree() { root = nullptr; }


        // ---------------------------------------------------
        //  Helper function used by destructor
        //  Deletes all nodes in BST
        //  Similar to DFS PostOrder in Tree Traversal section
        // ---------------------------------------------------
        void destroy(Node* currentNode) {
            if (currentNode) {
                destroy(currentNode->left);
                destroy(currentNode->right);
                delete currentNode;
            }
        }

        ~BinarySearchTree() { destroy(root); }
 

        void insert(int value) {
            Node* newNode = new Node(value);
            if (root == nullptr) {
                root = newNode;
                return;
            }
            Node* temp = root;
            while(true) {
                if (newNode->value == temp->value) return;
                if (newNode->value < temp->value) {
                    if (temp->left == nullptr) {
                        temp->left = newNode;
                        return;
                    }
                    temp = temp->left;
                } else {
                    if (temp->right == nullptr) {
                        temp->right = newNode;
                        return;
                    }
                    temp = temp->right;
                }
            }
        }

        bool contains(int value) {
            if (root == nullptr) return false;
            Node* temp = root;
            while(temp) {
                if (value < temp->value) {
                    temp = temp->left;
                } else if (value > temp->value) {
                    temp = temp->right;
                } else {
                    return true;
                }
            }
            return false;
        }

       void BFS() {
            Node* currentNode = root;
            queue<Node*> myQueue;
            myQueue.push(currentNode);

            while (myQueue.size() > 0) {
                currentNode = myQueue.front();
                myQueue.pop();
                cout << currentNode->value << " ";
                if (currentNode->left) {
                    myQueue.push(currentNode->left);
                }
                if (currentNode->right) {
                    myQueue.push(currentNode->right);
                }
            }
        }

        void DFSPreOrder(Node* currentNode) {
            cout << currentNode->value << " ";
            if (currentNode->left != nullptr) {
                DFSPreOrder(currentNode->left);
            }
            if (currentNode->right != nullptr) {
                DFSPreOrder(currentNode->right);
            }
        }

        void DFSPreOrder() { DFSPreOrder(root); }


        void DFSPostOrder(Node* currentNode) {
            if (currentNode->left != nullptr) {
                DFSPostOrder(currentNode->left);
            }
            if (currentNode->right != nullptr) {
                DFSPostOrder(currentNode->right);
            }
            cout << currentNode->value << " ";
        }

        void DFSPostOrder() { DFSPostOrder(root); }


        void DFSInOrder(Node* currentNode) {
            if (currentNode->left != nullptr) {
                DFSInOrder(currentNode->left);
            }
            cout << currentNode->value << " ";
            if (currentNode->right != nullptr) {
                DFSInOrder(currentNode->right);
            }
        }

        void DFSInOrder() { DFSInOrder(root); }

};



int main() {
        
    BinarySearchTree* myBST = new BinarySearchTree();

    myBST->insert(47);
    myBST->insert(21);
    myBST->insert(76);
    myBST->insert(18);
    myBST->insert(27);
    myBST->insert(52);
    myBST->insert(82);

    cout << "DFS InOrder:\n";
    myBST->DFSInOrder();

    /*
        EXPECTED OUTPUT:
        ----------------
        DFS InOrder:
        18 21 27 47 52 76 82  

    */    

}

