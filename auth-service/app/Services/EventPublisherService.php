<?php

namespace App\Services;

use PhpAmqpLib\Message\AMQPMessage;
use PhpAmqpLib\Channel\AMQPChannel;

class EventPublisherService
{
    protected $channel;

    public function __construct(AMQPChannel $channel)
    {
        $this->channel = $channel;
    }

    public function publishUserEvent(string $event, array $userData)
    {
        // Declare exchange
        $this->channel->exchange_declare('user_events', 'topic', false, true, false);
        
        $message = new AMQPMessage(
            json_encode([
                'event' => $event,
                'data' => $userData,
                'timestamp' => time()
            ]),
            ['content_type' => 'application/json', 'delivery_mode' => AMQPMessage::DELIVERY_MODE_PERSISTENT]
        );
        
        $this->channel->basic_publish($message, 'user_events', "user.$event");
    }
}