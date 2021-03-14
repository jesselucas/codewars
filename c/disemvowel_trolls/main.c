#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <stdlib.h>
#include <stdint.h>

// const char *vowels[] = {"a", "e", "i", "o", "u"};
const char *vowels = "aeiou";

// Solution must allocate all required memory
// and return a free-able buffer to the caller.
char *
disemvowel(const char *str)
{
	size_t len = strlen(str);

	// create array to hold answer
	size_t size = sizeof(char);
	char *r_str;
 	if (size && len > SIZE_MAX / size) 
		exit(-1);
        if ((r_str = malloc(len * size)) == NULL)
		exit(-1);

	// Loop through str
	int j=0;
	for(int i = 0; i < len; i++) {
		// Check each vowel
		char c = tolower(str[i]);
		if(c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'){
			continue;
		} else {
			r_str[j] = str[i];
			j++;
		}
	}
	return r_str;
}

int
main(void) 
{
	char *r_str = disemvowel("This website is for losers LOL!");
	printf("%s\n", r_str);
	printf("%i\n", strcmp(r_str, "Ths wbst s fr lsrs LL!") >= 0); 
	free(r_str);
	return 0;
}
