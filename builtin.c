#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>

// Function to swap two numbers

void swap(char *x, char *y) {
	char t = *x; *x = *y; *y = t;
}

// Function to reverse `buffer[i…j]`

char* reverse(char *buffer, int i, int j)
{
	while (i < j) {
		swap(&buffer[i++], &buffer[j--]);
	}
	return buffer;
}

// Iterative function to implement `itoa()` function in C

char* itostr(int value, char* buffer, int base)
{
	// invalid input
	if (base < 2 || base > 32) {
		return buffer;
	}

	// consider the absolute value of the number
	int n = abs(value);

	int i = 0;
	while (n)
	{
		int r = n % base;

		if (r >= 10) {
			buffer[i++] = 65 + (r - 10);
		}
		else {
			buffer[i++] = 48 + r;
		}

		n = n / base;
	}

	// if the number is 0
	if (i == 0) {
		buffer[i++] = '0';
	}

	// If the base is 10 and the value is negative, the resulting string
	// is preceded with a minus sign (-)
	// With any other base, value is always considered unsigned
	if (value < 0 && base == 10) {
		buffer[i++] = '-';
	}

	buffer[i] = '\0'; // null terminate string

	// reverse the string and return it
	return reverse(buffer, 0, i - 1);
}

int64_t i64(int32_t num){
	return (int64_t)num;
}

int16_t i16(int32_t num){
	return (int16_t)num;
}

char* bytearray(int32_t count){
	char* bytes = (char*)malloc(count * sizeof(char));
	return bytes;
}

// // Implement itoa function in C
// int main(void)
// {
// 	//char buffer[33];
// 	char buffer2[33];

// // 	int value[] = { 44556677 };
// // 	int base[]  = { 10 };

// // 	for (int i = 0; i < 4; i++)
// // 	{
// // 		printf("itoa(%d, buffer, %d) = %s\n", value[i], base[i],
// // 			itoa(value[i], buffer, base[i]));
// // 	}
//     //int b = 10;
//     //int i = 44556677;
//     //itoa(i,buffer,10);
    
// 	//str(44556677,buffer,10);
//     //printf ("decimal: %s\n", buffer);

// 	itostr(44556677,buffer2,10);
//     print(buffer2);

// 	return 0;
// }
