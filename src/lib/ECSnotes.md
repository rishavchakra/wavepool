# Entity
Entities uniquely identify a single item

Contains only an id number

# Component
Components hold the data describing behavior

Component Data are public parameters

Some types of components are static, some are dynamic
## Static
owned by the entity
> not actually owned by entity, owned by the controller and linked to a single entity

### Examples
* Name
* Transform/position (vector pos)
* Velocity (vector vel)

## Dynamic
not owned by the entity
> owned by the controller and linked to by the entities
### Examples
* Gravity (vector g, match global?)
* Collision (collider mesh)
* Scripts (script path)
* Sub-chains (Reference to Dynamic Component Chain)

# System
Performs functionality based on the components and their parameters, may hold necessary persistent data?

# Controller
Owns all data in the ECS model, performs functionality defined by the systems

## Data
* Entities: Set
* Dynamic Component Chains: Set
* Static Component Chains: Set
* Static Component Link: Map from Entity to Static Component Chain
* Dynamic Component Link: Map from Entity to Dynamic Component Chain ref
* Systems: Keys of Systemlink map
* System Link: Map of Systems to Entities
    * Systems are defined/hashed as equal if they have the same system type to prevent duplicate system functionality
    * Or are different structs implementing a System trait
