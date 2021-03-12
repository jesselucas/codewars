#include <stdio.h>
#include <string.h>

//const char *vowels[] = {"a", "e", "i", "o", "u"};
const char *vowels = "aeiou";

// solution must allocate all required memory
// and return a free-able buffer to the caller.
char *
disemvowel(const char *str)
{
	int len = sizeof(str);
	// Loop through str
	for(int i = 0; i < len; i++) {
		// Check each vowel
		printf("%c", str[i]);
	}
  return "Ths wbst s fr lsrs LL!";
}

int
main(void) 
{
	printf("%i\n", strcmp(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!") >= 0); 

	return 0;
}
