#include <iostream>
#include <string>
#include <memory>  // REQUIRED for std::unique_ptr
#include <ctime>
#include <cstdlib>

class Exam {
    private:
        std::string examName;
        // A smart pointer to an array of smart pointers
        std::unique_ptr<std::unique_ptr<double>[]> examScores;
        int capacity;
        int size;

    public:
        // Use std::make_unique to safely allocate the array
        Exam(std::string name, int cap) : examName(name), capacity(cap), size(0) {
            examScores = std::make_unique<std::unique_ptr<double>[]>(capacity);
        }

        // NO DESTRUCTOR NEEDED 
        // When 'examScores' goes out of scope, it automatically cleans up everything.

        void addScore(double score) {
            if (size == capacity) resize(capacity * 2);
            
            // Allocate the individual double as a unique_ptr
            examScores[size] = std::make_unique<double>(score);
            size++;
        }

        void resize(int newCap) {
            std::cout << "\n[!] Expanding " << examName << " to capacity: " << newCap << "...\n";
            
            // 1. Create the new array
            auto newArray = std::make_unique<std::unique_ptr<double>[]>(newCap);

            // 2. MOVE the ownership of the pointers
            for (int i = 0; i < size; i++) {
                // std::move is required because unique_ptr cannot be copied, only moved
                newArray[i] = std::move(examScores[i]);
            }
            
            // 3. Swap the old array for the new one
            // The old 'examScores' array is automatically deleted here!
            examScores = std::move(newArray);
            capacity = newCap;
        }

        void display() {
            std::cout << "\n--- " << examName << " Gradebook ---" << std::endl;
            for (int i = 0; i < size; i++) {
                // We still dereference to get the value
                std::cout << "Student " << i + 1 << ": " << *examScores[i] << "%" << std::endl;
            }
        }

        int getCapacity() const { return capacity; }
};

int main() {
    srand(time(0));
    
    Exam biology("Bio 101", 10);
    Exam math("Math 101",10);

    for (int i = 0; i < 100; i++) {
        double randomScore = 60.0 + (rand() % 351) / 10.0; 
        biology.addScore(randomScore);
    }
    for (int i = 0; i < 100; i++) {
        double randomScore = 60.0 + (rand() % 351) / 10.0; 
        math.addScore(randomScore);
    }

    biology.display();
    math.display();

    return 0; // Everything is cleaned up automatically here.
}
