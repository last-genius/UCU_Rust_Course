int test(int* pointer)
{
	*pointer = 5;
	return 0;
}

int main() 
{
	char* text = malloc(10);
	*text = 'c';
	return 0;
}
