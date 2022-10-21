<?php

class A {
    private $testing = "test";

    public function testing(string $a): void{
        if($a === 'test'){
            echo 'Hello!';
        }
    }
}