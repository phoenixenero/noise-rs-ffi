#include <stdint.h>

typedef struct Seed Seed;

Seed *noise_seed_new(uint32_t seed);
void noise_seed_delete(Seed *seed);

/* Perlin noise */
double noise_perlin2(Seed *seed, double x, double y);
double noise_perlin3(Seed *seed, double x, double y, double z);
double noise_perlin4(Seed *seed, double x, double y, double z, double w);

/* OpenSimplex noise */
double noise_open_simplex2(Seed *seed, double x);
double noise_open_simplex3(Seed *seed, double x, double y);

/* Cell noise (euclidean distance) */
double noise_cell2_value(Seed *seed, double x, double y);
double noise_cell3_value(Seed *seed, double x, double y, double z);
double noise_cell4_value(Seed *seed, double x, double y, double z, double w);

double noise_cell2_range(Seed *seed, double x, double y);
double noise_cell3_range(Seed *seed, double x, double y, double z);
double noise_cell4_range(Seed *seed, double x, double y, double z, double w);

double noise_cell2_range_inv(Seed *seed, double x, double y);
double noise_cell3_range_inv(Seed *seed, double x, double y, double z);
double noise_cell4_range_inv(Seed *seed, double x, double y, double z, double w);

/* Cell noise (Manhattan distance) */
double noise_cell2_manhattan_value(Seed *seed, double x, double y);
double noise_cell3_manhattan_value(Seed *seed, double x, double y, double z);
double noise_cell4_manhattan_value(Seed *seed, double x, double y, double z, double w);

double noise_cell2_manhattan_range(Seed *seed, double x, double y);
double noise_cell3_manhattan_range(Seed *seed, double x, double y, double z);
double noise_cell4_manhattan_range(Seed *seed, double x, double y, double z, double w);

double noise_cell2_manhattan_range_inv(Seed *seed, double x, double y);
double noise_cell3_manhattan_range_inv(Seed *seed, double x, double y, double z);
double noise_cell4_manhattan_range_inv(Seed *seed, double x, double y, double z, double w);
