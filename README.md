# Rust Data Service
Objective of building this service is to deploy-and-forget. A modern backend stack combining the type-safety of Rust, the flexibility of GraphQL, and the reliability of Postgres.

### Setup
<ul>
  <li>Install Rust and Configure Cargo Package Manager</li>
  <li>Download Zip/Clone</li>
  <li>Configure database connection in .env file as per instruction in the <b>Configuration</b> section</li>
  <li>run command cargo run </li>
</ul>


### Configuration
Change the configuration to connect database in .env file
<table>
  <thead>
    <tr>
      <th>Key</th>
      <th>Value</th>
      <th>Comments</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>
        PORT
      </td>
      <td>
        80
      </td>
      <td>
        Port can be changed
      </td>
    </tr>
    <tr>
      <td>
        DATABASE_URL
      </td>
      <td>
        postgres://postgres:<password>@localhost:5432/dbname
      </td>
      <td>
        Database host can be changed
      </td>
    </tr>
    
  </tbody>
</table>

### Queries
<ol>
  <li>select</li>
  <li>count</li>
 <!--  <li>tables</li>
  <li>columns</li>
  <li>foreignkeys</li>
  <li>distinct</li>
  <li>avg</li>
  <li>max</li>
  <li>min</li>
  <li>sum</li> -->
</ol>

<!-- ### Mutation
<ol>
  <li>insert</li>
  <li>update</li>
  <li>delete</li>
  <li>simple - database transaction</li>
</ol> -->