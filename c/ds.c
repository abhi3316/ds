#include <stdio.h>


int binary_search(const int *arr, int len, const int num) {
	int start = 0;
	int end = len - 1;
	int middle = 0;
	for(;start <= end;) {
		middle = (start + end) / 2;
		if(arr[middle] == num) {
			return (middle);
		} else if (num > arr[middle]) {
			start = middle + 1;
		} else {
			end = middle - 1;
		}
	}

	return -1;
}	

int main(void) {
	int arr[] = {1, 2, 3, 4, 5, 6, 7};

	for(int i = 0; i < sizeof(arr)/ sizeof(arr[0]); i++) {
		int num = arr[i];
		const int pos = binary_search(arr, sizeof(arr)/sizeof(arr[0]), num);
		if(pos == i) {
			printf("%d , TEST PASSED\n", arr[i]);
		}
	}
	return 0;
}
