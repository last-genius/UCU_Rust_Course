void main() 
{
	Vec*vec=vec_new();
	vec_push(vec, 107);

	int* n = &vec->data[0];
	vec_push(vec,110);
	printf("%d\n", *n);

	free(vec->data);  
	vec_free(vec);
}
