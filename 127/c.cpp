#include <iostream>
using namespace std;

int main(void) {
  	int n, m;
	cin >> n >> m;
  	int l[m], r[m];
  	for (int i=0;i<m;i++) {
    	cin >> l[i] >> r[i];
      	if (i>0) {
          l[0] = max(l[i], l[0]);
          r[0] = min(r[i], r[0]);
        }
    }
    int distance;
    distance = (r[0] >= l[0]) ? (r[0]-l[0]+1) : 0;
  	cout << distance  << endl;
  	return 0;
}
