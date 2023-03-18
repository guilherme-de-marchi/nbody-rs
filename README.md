# mds-rs
A molecular dynamics simulator written in Rust

# Mathematical background

The resulting force acting on an atom is composed by:

    - It's velocity and acceleration;
    - The sum of interatomic forces acting on that atom;

## Interatomic interaction

Every atom will apply a force on every atom of the system.
The force applied by the atom A on atom B will be applied
by atom B on A with the same magnitude, but inverse direction.
So, the interatomic energy applied by A on B is proportional to its distance.

### Interatomic energy

There are different contributions to the energy of interation
between atoms A and B:

    - Coulomb's law: Ec = Ke |q1| |q2| / r^2
    Here, Ke is the Coulomb constant (ke ≈ 8.988×10^9 N⋅m2⋅C−2), q1 and q2 are the assigned magnitudes of the charges, and the scalar r is the distance between the charges;

    - Van der Waals force: Ev = -1 / r^6

    - Lennard-Jones potential: El = 1 / r^12 

The negative derivative of the sum of these three functions in respect to the distance between the atoms times a unit vector is the force applied by atom A on B: F = -(Ec + Ev + El)' * U . The sum of every F applied on B is the resultant force applied on B.

# Sources

- [Basics of Molecular Dynamics Simulations, PARISlab@UCLA](https://www.youtube.com/watch?v=ipRnvs7_CxA)