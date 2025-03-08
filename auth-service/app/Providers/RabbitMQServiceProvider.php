<?php

namespace App\Providers;

use Illuminate\Support\ServiceProvider;
use PhpAmqpLib\Connection\AMQPStreamConnection;
use PhpAmqpLib\Channel\AMQPChannel;

class RabbitMQServiceProvider extends ServiceProvider
{
    public function register()
    {
        $this->app->singleton('rabbitmq.connection', function ($app) {
            return new AMQPStreamConnection(
                config('rabbitmq.host'),
                config('rabbitmq.port'),
                config('rabbitmq.user'),
                config('rabbitmq.password'),
                config('rabbitmq.vhost')
            );
        });

        $this->app->singleton('rabbitmq.channel', function ($app) {
            return $app->make('rabbitmq.connection')->channel();
        });
    }

    public function provides()
    {
        return ['rabbitmq.connection', 'rabbitmq.channel'];
    }
}