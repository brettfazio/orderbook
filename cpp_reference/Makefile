all:
	g++ -O3 -c engine.cpp -std=c++2a
	g++ -O3 test.cpp -o test -std=c++2a
	g++ -O3 score.cpp -lm -o score -std=c++2a

# gcc -O3 score.c -lm -lrt -o score

clean:
	rm -f engine.o test score a.out *~
