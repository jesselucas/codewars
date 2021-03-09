#include <stdlib.h>
#include <stdio.h>

int ticket_price = 25;
int cash_register[256];

int 
tickets(size_t length, const int people[length]) {
	int ticketed = 0;
	if (people[0] != ticket_price) {
		return 0;
	}
	// add each person's purchase to the cash 
	for(int i = 0; i < length; i++) {
		int cash = people[i];	
		int change_due = people[i] - ticket_price;

		if (change_due == 0) {
			// add to cash
			cash_register[i] = cash;
			ticketed++;
			printf("no change due ticketed %d\n", ticketed);
			continue;
		}

		// TODO: sort by large bills to not use small bills to break
		// Check all cash in register	
		for(int j = 0; j < ticketed; j++) {
			if (cash_register[j] <= change_due) {
				// remove from cash and return true
				printf("found the change! %d %d\n", cash_register[j], ticketed);
				change_due = change_due - cash_register[j];
				cash_register[j] = 0; // remove from register
				if (change_due == 0) {
					ticketed++; // increment ticketed
					break;	
				}
			} 
		}	

		// add to cash
		cash_register[i] = cash;
	}
	printf("ticketed? %i\n", ticketed);
	if (ticketed == length) {
		return 1;
	}
	return 0; // instead of strings "YES"/"NO" return true/false instead
}

int 
main(void) {
	printf("%d\n", tickets(3, (int[]){50, 25, 50})); // 0
	printf("%d\n", tickets(3, (int[]){25, 25, 50})); // 1
	printf("%d\n", tickets(6, (int[]){25, 25, 25, 25, 50, 50})); // 1
	printf("%d\n", tickets(8, (int[]){25, 25, 25, 25, 50, 50, 25, 100})); // 1
	printf("%d\n", tickets(2, (int[]){25, 100})); // 0
	printf("%d\n", tickets(7, (int[]){25, 25, 25, 25, 50, 100, 50}));

	return 0;
}
