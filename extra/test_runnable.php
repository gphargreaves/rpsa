<?php

$code = file_get_contents('/Users/greg/rpsa/extra/test.php');

$tokens = PhpToken::tokenize($code);

foreach ($tokens as $token) {
    echo "Line {$token->line}: {$token->getTokenName()}", PHP_EOL;
}