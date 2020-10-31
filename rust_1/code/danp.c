int *test()
{  
	int y=10;  
	return &y;  
}  

int main()  
{  
	int *p = test();  
	printf("%d", *p);  
	return 0;  
}  
