
typedef struct Vector {
    int a;
    int b;
} Vector;

extern int extract_a(Vector v) {
    return v.a;
}
