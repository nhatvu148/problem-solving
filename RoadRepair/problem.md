```
A number of points along the highway are in need of repair. an equal number of crews are available, stationed at various points along the highway. they must move along the highway to reach an assigned point. given that one crew must be assigned to each job, what is the minimum total amount of distance traveled by all crews before they can begin work? for example, given crews at points (1,3,5) and required repairs at (3,5,7), one possible minimum assignment would be (1-3, 3 - 5, 5-7) for a total of 6 units traveled.
```

```
There are n cities and there are roads in between some of the cities. Somehow all the roads are damaged simultaneously. We have to repair the roads to connect the cities again. There is a fixed cost to repair a particular road. Find out the minimum cost to connect all the cities by repairing roads. Input is in matrix(city) form, if city[i][j] = 0 then there is not any road between city i and city j, if city[i][j] = a > 0 then the cost to rebuild the path between city i and city j is a. Print out the minimum cost to connect all the cities.

It is sure that all the cities were connected before the roads were damaged.Examples:

Input : {{0, 1, 2, 3, 4},

{1, 0, 5, 0, 7},

{2, 5, 0, 6, 0},

{3, 0, 6, 0, 0},

{4, 7, 0, 0, 0}};

Output : 10

Input : {{0, 1, 1, 100, 0, 0},

{1, 0, 1, 0, 0, 0},

{1, 1, 0, 0, 0, 0},

{100, 0, 0, 0, 2, 2},

{0, 0, 0, 2, 0, 2},

{0, 0, 0, 2, 2, 0}};

Output : 106
```

```
Greedy sorting works based on the given test-case constraints. You can try advanced graph algos ,  which takes more than O(n^3) and fails(TLE) for given constraints. The greedy sorting algo has T.C = O(n*logn) which perfectly works for given test-case constraints. (Note: Question was asked in hackerank contest and n can be as large as 10^6)

Explanation:

Actually, you don't need any advanced Graph algorithms like Hungarian, MST, or Max bipartite matching. Just greedy sorting works because as all points are on a single line and matching the minimum with minimum guarantees you the minimum possible cost/distance and this is the main core idea of any greedy based approach.

Refer to below pseudocode in C++ for a full clear picture.

int prob( vector<int> crew ,  vector<int> job, int N)

{

  sort(crew.begin() , crew.end());

  sort(job.begin() , job.end());

  int min_val = 0;

  for(int i = 0 ; i < N ; i++)

  {

      int match_cost = abs(crew[i] - job[i]); //distance is absolute

      min_val += match_cost;

  }

  return min_val;

}
```
