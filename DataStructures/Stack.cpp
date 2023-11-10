#include <iostream>

using namespace std;


class Node { 
    public: 
        int value;
        Node* next;
    
        Node(int value) {
            this->value = value;
            next = nullptr;
        }
};


class Stack {
    private:
        Node* top;
        int height;

    public:
        Stack(int value) {
            Node* newNode = new Node(value);
            top = newNode;
            height = 1;
        }

        ~Stack() {
            Node* temp = top;
            while (top) {
                top = top->next;
                delete temp;
                temp = top;
            }
        }

        void printStack() {
            Node* temp = top;
            while (temp) {
                cout << temp->value << endl;
                temp = temp->next;
            }
        }

        void getTop() {
            if (top == nullptr) {
                cout << "Top: nullptr" << endl;
            } else {
                cout << "Top: " << top->value << endl;
            }
        }

        void getHeight() {
            cout << "Height: " << height << endl;
        }

        bool isEmpty() {
            if (height == 0) return true;
            return false;
        }

        void push(int value) {
            Node* newNode = new Node(value);
            newNode->next = top;
            top = newNode;
            height++;
        }

        int topValue() { 
            if (top) return top->value;
            return INT_MIN;
        }

        int pop() {
            if (height == 0) return INT_MIN;

            Node* temp = top;
            int poppedValue = top->value;
            top = top->next;
            delete temp;
            height--;

            return poppedValue;
        }
            
};



int main() {
        
    Stack* myStack = new Stack(1);   

    cout << "Popped value: " << myStack->pop();

    cout << "\n\nPopped value: " << myStack->pop();


    /*  
        EXPECTED OUTPUT:
        ----------------
        Popped value: 1

        Popped value: -2147483648
    
    */
            
}  

