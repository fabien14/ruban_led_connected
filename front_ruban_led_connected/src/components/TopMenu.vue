<template>
    <nav class="navbar  navbar-expand-lg navbar-dark bg-dark">
        <div class="container-fluid">
            <a class="navbar-brand" href="#">Navbar</a>
            <div class="collapse navbar-collapse">
                <ul class="navbar-nav">
                    <li class="nav-item">
                        <router-link to="/" class="nav-link active" aria-current="page" >Home</router-link>
                    </li>
                    <li class="nav-item">
                        <router-link to="/devices" class="nav-link active" aria-current="page" >Devices</router-link>
                    </li>
                    <li class="nav-item">
                        <router-link to="/devices/1" class="nav-link active" aria-current="page" >Device</router-link>
                    </li>
                </ul>
            </div>
        </div>
    </nav>
</template>