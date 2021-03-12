#include <stdlib.h>
#include <stdio.h>

#define REGISTER_LENGTH 256

int ticket_price = 25;
int cash_register[REGISTER_LENGTH];

void
empty_register(int length, int *array) 
{
	if (length > REGISTER_LENGTH) {
		return;
	}
	for(int i = 0; i < length; i++) {
		array[i] = 0;
	}
}
void
order_register(int length, int *array)
{
	if (length > REGISTER_LENGTH) {
		return;
	}
	for(int i = 0; i < length; i++) {
		if (array[i] == 0) {
			// Shift everything to the left
			for(int j = i; j < length; j++) {
				array[j] = array[j + 1];
			}
		}
	}
}


void
remove_cash_at(int length, int *array, int index) 
{
	if (length > REGISTER_LENGTH) {
		return;
	}

	// Set to 0 and shift everyting after to the left
	array[index] = 0;
	
	for(int i = index; i < length; i++) {
		array[i] = array[i + 1];
	}
}

void
insert_cash_at(int length, int *array, int index, int value) 
{
	// Shift to the right to make room 
	// Start from end so we don't overwrite values
	for(int i = length - 1; i >= index; i--) {
		array[i] = array[i - 1];	
	}	
	array[index] = value;
}

int
add_cash(int length, int cash) {
	if (length > REGISTER_LENGTH) {
		return -1;
	}
	// add biggest bills at front
	for (int i = 0; i < length; i++) {
		// Find a register that is less than the cash
		// then move all the other cash over
		if(cash_register[i] < cash) {
			// insert cash at that index 
			insert_cash_at(length, cash_register, i, cash);
			return 0;
		}
	}	
	
	return 0;
}

int 
tickets(size_t length, const int people[length]) {
	empty_register(REGISTER_LENGTH, cash_register);

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
			// cash_register[i] = cash;
			int err = add_cash(length, cash);
			if (err < 0) {
				return 0;	
			}
			ticketed++;
			continue;
		}

		// Check all cash in register	
		for(int j = 0; j < ticketed; j++) {
			if (cash_register[j] <= change_due) {
				change_due = change_due - cash_register[j];
				cash_register[j] = 0; // remove from register
				// remove_cash_at(ticketed, cash_register, j);
				if (change_due == 0) {
					ticketed++; // increment ticketed
					break;	
				}
			} 
		}	
		// Order cash 
		order_register(ticketed, cash_register);	

		// printf("ticketed %i\n", ticketed);
		// add to cash
		int err = add_cash(length, cash);
		if (err < 0) {
			printf("add cash error\n");
			return 0;	
		}
	}
	if (ticketed == length) {
		return 1;
	}
	return 0; // instead of strings "YES"/"NO" return true/false instead
}

int 
main(void) {
	printf("Expected: 0, Result: %d\n", tickets(3, (int[]){50, 25, 50})); // 0
	printf("Expected: 1, Result: %d\n", tickets(3, (int[]){25, 25, 50})); // 1
	printf("Expected: 1, Result: %d\n", tickets(4, (int[]){25, 25, 50, 100})); // 1
	printf("Expected: 1, Result: %d\n", tickets(6, (int[]){25, 25, 25, 25, 50, 50})); // 1
	printf("Expected: 1, Result: %d\n", tickets(8, (int[]){25, 25, 25, 25, 50, 50, 25, 100})); // 1
	printf("Expected: 0, Result: %d\n", tickets(2, (int[]){25, 100})); // 0
	printf("Expected: 1, Result: %d\n", tickets(7, (int[]){25, 25, 25, 25, 50, 100, 50})); // 1
	printf("Expected: 1, Result: %d\n", tickets(17, (int[]){25, 25, 25, 50, 25, 50, 50, 25, 100, 25, 25, 100, 50, 100, 25, 50, 25}));

	/*
	for(int i=0; i < 5; i++) {
		printf("value: %i\n", cash_register[i]);
	}
	*/

	return 0;
}
